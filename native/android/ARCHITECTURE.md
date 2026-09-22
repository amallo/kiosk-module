# Architecture : `native/android` (pont Rust ↔ Android)

Ce document décrit comment la logique métier pure du crate [`native/core`](../core) est
exposée sur Android, et les décisions d'architecture qui en découlent. Il complète les
commentaires du code — à mettre à jour si l'une de ces décisions change.

## Vue d'ensemble

```
JS (React Native)
   │  (à faire : ajouter lockDevice/grantTime à src/KioskModule.nitro.ts + yarn nitrogen)
   ▼
Kotlin — KioskModule.kt (android/src/main/java/.../kioskmodule/)
   │  System.loadLibrary("kiosk_android") via KioskNative.kt
   ▼
JNI — native/android/src/lib.rs (crate `kiosk-android`, cdylib)
   │  nativeInit / nativeLockDevice / nativeGrantTime / nativeDestroy
   ▼
Composition root — native/android/src/app_context.rs (AppContext)
   │  instancie une seule fois : runtime tokio + use cases + adapters concrets
   ▼
native/core — logique métier pure (LockDeviceUseCase, GrantTimeUseCase, traits)
```

`core` reste un `rlib` pur, sans aucune dépendance JNI/Android — tout le code
spécifique à la plateforme vit dans `native/android`.

## Pourquoi un crate séparé (`native/android`) plutôt que JNI dans `core`

- `core` doit rester utilisable tel quel côté iOS (ou tout autre binding futur).
- Sépare clairement logique métier (testable sans JVM, `cargo nextest`) et code
  d'infrastructure (JNI, cycle de vie Android).

## Composition root : un handle natif = un `AppContext`

Les use cases et leurs adapters concrets sont instanciés **une seule fois** par
handle, pas à chaque appel JNI. `nativeInit()` construit un `AppContext` (runtime
tokio `current_thread` + use cases + adapters) et retourne un pointeur opaque
(`jlong`) au Kotlin appelant. Ce dernier le repasse à chaque appel
(`nativeLockDevice`, `nativeGrantTime`) et doit appeler `nativeDestroy()` une fois
terminé (câblé sur `HybridObject.dispose()` côté `KioskModule.kt`).

`Box<AppContext>` (pas `Arc`) : un seul propriétaire pour ce handle, cycle de vie
déterministe (créé dans `nativeInit`, détruit dans `nativeDestroy`) — le pattern
FFI classique d'« opaque pointer ». `Arc` n'apporterait rien ici puisqu'il n'existe
qu'une seule référence vivante à la fois ; il deviendrait pertinent si `AppContext`
devait un jour être partagé avec une tâche de fond vivant indépendamment du handle
JNI.

## Adapters concrets (premier jet)

| Trait | Implémentation | Pourquoi |
|---|---|---|
| `Clock` | `SystemClock` | `SystemTime`, trivial |
| `PinValidator` | `SimplePinValidator` | PIN fixe passé en construction — provenance définitive (config/chiffré) non traitée |
| `DeviceLocker` | `LoggingDeviceLocker` | **Stub volontaire** : logue les appels (`adb logcat`) sans agir réellement. Valide tout le pipeline JNI sans dépendre de `DevicePolicyManager`/Device Owner |
| `TimeCreditStorage` | `FileTimeCreditStorage` | JSON persistant sur disque (écriture atomique tmp+rename), **choisi plutôt qu'un stockage en RAM** pour deux raisons : survivre à un reload de l'app, et servir de point de synchronisation avec le futur déclenchement AlarmManager (voir plus bas) |

## Pièges rencontrés

- **Ne jamais nommer une dépendance Cargo `core`** dans un crate qui utilise
  `async_trait` : ça entre en collision avec le crate `core` du langage (edition
  2018+), et les macros générées (`::core::pin::Pin`, etc.) résolvent vers la
  mauvaise cible. Solution : aliaser dans `Cargo.toml` :
  ```toml
  kiosk_core = { path = "../core", package = "core" }
  ```
- **`cargo-ndk` 4.1.2** ne résout pas `--manifest-path` correctement si la
  commande est lancée depuis un autre répertoire que celui du crate. Il faut se
  placer dans `native/android` et utiliser un chemin de sortie relatif :
  ```bash
  cd native/android
  cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -t x86 \
    -o ../../android/src/main/jniLibs build --release
  ```
  (script npm : `yarn build:rust:android`)

## Ce qui n'est pas encore fait

- Exposer `lockDevice`/`grantTime` côté JS (ajouter à `src/KioskModule.nitro.ts`,
  régénérer avec `yarn nitrogen`).
- Intégration CI (`.github/workflows/ci.yml`) et `turbo.json`.
- Test de bout en bout dans `example/android`.
- Verrouillage réel de l'appareil (remplacer `LoggingDeviceLocker`).

## Prochain chantier : déclenchement de `LockDeviceUseCase` par `AlarmManager`

Décision produit : `GrantTimeUseCase` est déclenché par l'app (action utilisateur,
PIN), `LockDeviceUseCase` est déclenché côté Android par un `AlarmManager` —
indépendamment de la présence de l'app au premier plan. `GrantTimeUseCase` a besoin
d'un utilisateur ; `LockDeviceUseCase` n'a besoin que de l'horloge et du storage,
donc se prête à un déclenchement système.

Points à respecter lors de l'implémentation :

1. **Deux cycles de vie de handle distincts.** Le `BroadcastReceiver` réveillé par
   l'alarme peut tourner dans un process relancé à froid (app tuée par l'OS) — il
   ne peut pas réutiliser le `nativeHandle` long-vécu de `KioskModule`. Il doit
   faire son propre cycle court à chaque réveil :
   `nativeInit()` → `nativeLockDevice()` → `nativeDestroy()`.
2. **`FileTimeCreditStorage` (et non `InMemory`) est ce qui rend ça possible** :
   c'est le seul point de synchronisation entre le handle long-vécu de l'app et
   les handles courts du receiver (même chemin `filesDir`).
3. **`schedule_lock(end)` (appelé par `GrantTimeUseCase`) doit réellement
   programmer l'alarme Android**, pas juste logger comme le fait
   `LoggingDeviceLocker` aujourd'hui. Nécessite un vrai `DeviceLocker` qui rappelle
   Kotlin via JNI (callback + `GlobalRef`/`JavaVM`) pour poser un
   `PendingIntent`/`AlarmManager`.
4. **Alarme ponctuelle (one-shot), pas un polling périodique** :
   `grantTime(duration=15min)` programme une seule alarme à `now+15min`.
5. **Une seule alarme en attente à la fois** : si l'utilisateur regrant du temps
   avant l'expiration du crédit précédent, la nouvelle alarme doit remplacer
   l'ancienne (même `requestCode`/`PendingIntent`), sinon l'alarme obsolète
   verrouillerait prématurément l'appareil.
6. **Un reboot efface les alarmes AlarmManager mais pas le fichier de crédit.**
   Prévoir un receiver `BOOT_COMPLETED` qui relit le fichier et reprogramme
   l'alarme si un crédit est encore actif.
7. **Process séparé (`android:process=":lock"`) pour le receiver : à évaluer**
   selon si `MainApplication.onCreate()` initialise RN/Hermes/Nitro de façon
   eager (un process séparé évite de réveiller tout le runtime JS pour un simple
   check). Contrepartie : aucun état partagé en RAM entre les deux process
   (uniquement via le fichier), et `NitroModules.applicationContext` serait `null`
   dans ce process (récupérer le `Context` depuis `onReceive(context, intent)` à
   la place).

Pas encore implémenté — chantier dédié à venir (nouveau `BroadcastReceiver`,
`AlarmScheduler` Kotlin, vrai `DeviceLocker` Android, receiver `BOOT_COMPLETED`).
