# Développement natif : build & ajout d'un use case

Ce guide complète [`CONTRIBUTING.md`](../CONTRIBUTING.md) sur deux points spécifiques à ce projet : le build complet de l'app example (avec le cœur Rust), et la procédure pour ajouter un nouveau use case métier en Rust et l'exposer jusqu'en JS/TS.

## Prérequis

- Node.js + Yarn (voir `.nvmrc`).
- Toolchain Rust (`rustup`) + [`cargo-ndk`](https://github.com/bbqsrc/cargo-ndk) pour builder le crate Android (`cargo install cargo-ndk`).
- Android NDK installé (via Android Studio) et `ANDROID_NDK_HOME` configuré.
- Un émulateur ou device Android connecté (`adb devices`).

## Build de l'app example (Android)

Le module natif Android repose sur **deux bibliothèques `.so` distinctes** :

1. `libkioskmodule.so` — glue Nitro/C++, générée automatiquement par CMake pendant le build Gradle.
2. `libkiosk_android.so` — le crate Rust métier (`native/android`), qui doit être **construit manuellement** avant le build Android car son dossier de sortie (`android/src/main/jniLibs/`) est gitignored.

Étapes, depuis la racine du repo :

```sh
# 1. Installer les dépendances (une fois)
yarn

# 2. Régénérer les bindings Nitro si src/KioskModule.nitro.ts a changé
#    (obligatoire aussi au premier clone, les fichiers générés ne sont pas commités)
yarn nitrogen

# 3. Builder le crate Rust pour toutes les ABI Android et copier les .so
#    dans android/src/main/jniLibs/
yarn build:rust:android

# 4. Builder et lancer l'app example sur l'émulateur/device connecté
cd example && yarn android
```

Raccourci équivalent aux étapes 3-4 (utilisé en CI), depuis la racine :

```sh
yarn turbo run build:android
```

> ⚠️ Si `grantTime`/`lockDevice` sont appelées sans avoir lancé `yarn build:rust:android` au préalable, l'app plantera avec une `UnsatisfiedLinkError` (le `.so` Rust est absent).

### iOS

Il n'existe pas encore de pont Rust/FFI côté iOS : `ios/KioskModule.swift` ne fait qu'implémenter (ou stubber avec une erreur explicite) le protocole généré par Nitro. Pas de build Rust nécessaire pour iOS aujourd'hui.

```sh
cd example && yarn ios
```

## Ajouter un nouveau use case Rust et l'appeler depuis TS

La chaîne complète traverse 4 couches : **Rust core → JNI Android → Kotlin → spec Nitro (TS)**. Voici la procédure, illustrée avec l'exemple réel de `GrantTimeUseCase` déjà présent dans le repo.

### 1. Écrire le use case dans `native/core`

Créer `native/core/src/usecases/<nom>_use_case.rs` : une struct avec ses dépendances (adapters génériques via traits), une méthode `execute(...)` async retournant `Result<T, UseCaseError>`, et des tests unitaires avec des adapters de test (mocks/spies) — voir `native/core/src/usecases/grant_time_use_case.rs` comme modèle.

### 2. Instancier le use case côté Android (composition root)

Dans `native/android/src/app_context.rs` : instancier le use case avec ses adapters concrets (ex. `FileTimeCreditStorage`, `SystemClock`, `SimplePinValidator`, `LoggingDeviceLocker`) et l'ajouter comme champ de `AppContext`.

### 3. Exposer une fonction JNI

Dans `native/android/src/lib.rs` : ajouter une fonction `Java_com_margelo_nitro_kioskmodule_KioskNative_native<Nom>` qui :
- récupère le `AppContext` depuis le `handle` (`jlong`),
- exécute le use case via `ctx.runtime.block_on(...)` (le runtime tokio est mono-thread, l'appel est donc **synchrone/bloquant** du point de vue JNI),
- le tout enveloppé dans `panic::catch_unwind` pour ne jamais laisser un panic Rust remonter en JNI,
- mappe le `Result` en code entier via `native/android/src/jni_bridge.rs` (ajouter de nouvelles constantes `RESULT_*`/`ERR_*` et une fonction `map_*_result` si le use case introduit de nouveaux cas).

### 4. Rebuilder le crate Rust

```sh
yarn build:rust:android
```

### 5. Déclarer le binding externe côté Kotlin

Dans `android/src/main/java/com/margelo/nitro/kioskmodule/KioskNative.kt` : ajouter `external fun native<Nom>(...)` avec la signature JNI correspondante.

### 6. Ajouter la méthode au spec Nitro (contrat JS)

Dans `src/KioskModule.nitro.ts` : ajouter la méthode à l'interface `KioskModule`. Seuls les types supportés par Nitro sont utilisables (`number`, `string`, `boolean`, etc. — pas de `Long`/`Int` natifs, tout `number` TS devient un `Double` côté Kotlin/Swift).

### 7. Régénérer les bindings Nitro

```sh
yarn nitrogen
```

Cela régénère `nitrogen/generated/...` (dont `HybridKioskModuleSpec.kt`/`.swift`) — **ne jamais éditer ces fichiers à la main**, ils sont marqués `DO NOT MODIFY`.

### 8. Implémenter la méthode générée côté Kotlin

Dans `android/src/main/java/com/margelo/nitro/kioskmodule/KioskModule.kt` : ajouter `override fun <nom>(...)`, en convertissant les `Double` reçus vers les types attendus par le binding JNI (`Long`/`Int`), et en reconvertissant le code retour en `Double`.

### 9. Implémenter (ou stubber) côté iOS

Dans `ios/KioskModule.swift` : implémenter la méthode si un pont Rust/FFI iOS existe, sinon lever une erreur explicite (`NSError` avec message clair), comme fait pour `grantTime`, pour ne pas casser la compilation du protocole généré.

### 10. Exposer un wrapper JS

Créer `src/<nom>.native.tsx` (appel réel via `NitroModules.createHybridObject<KioskModule>('KioskModule').<nom>(...)`) et `src/<nom>.tsx` (fallback non-natif, ex. web), puis exporter depuis `src/index.tsx`. Voir `src/grantTime.native.tsx`/`src/grantTime.tsx` comme modèle.

### 11. Tester de bout en bout

```sh
yarn build:rust:android
cd example && yarn android
```

Appeler la nouvelle méthode depuis `example/src/App.tsx` et vérifier le comportement sur l'émulateur/device (succès et cas d'erreur).

## Codes retour actuels du pont JNI Android

Définis dans `native/android/src/jni_bridge.rs` :

| Code   | Constante                | Signification                          |
|--------|--------------------------|-----------------------------------------|
| `0`    | `RESULT_OK`               | Succès générique                        |
| `1`    | `RESULT_GRANTED`           | Temps accordé                           |
| `2`    | `RESULT_DENIED`            | Accès refusé                            |
| `-1`   | `ERR_TIME_CREDIT_STORAGE`  | Échec de lecture/écriture du crédit     |
| `-2`   | `ERR_PIN_VALIDATION`       | Code PIN invalide                       |
| `-3`   | `ERR_LOCK_DEVICE`          | Échec de verrouillage/déverrouillage    |
| `-100` | `ERR_INTERNAL_PANIC`       | Panic Rust interceptée                  |
