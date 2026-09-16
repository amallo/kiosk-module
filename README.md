# 📱 react-native-kiosk-module

Un module React Native pour transformer un appareil en **borne kiosque** : verrouillage de l'écran et déverrouillage temporisé protégé par code PIN.

L'idée est simple : un appareil (tablette, borne, terminal partagé...) reste verrouillé par défaut, et on lui accorde un **crédit de temps** déverrouillé uniquement après validation d'un code PIN. Une fois le temps écoulé, l'appareil se reverrouille automatiquement.

## ✨ Fonctionnalités

- 🔒 **Verrouillage de l'appareil** — bloque l'accès à l'appareil à la demande.
- 🔓 **Octroi de temps** — déverrouille l'appareil pour une durée donnée, après vérification d'un code PIN.
- 🧠 **Cœur métier natif en Rust** — la logique de verrouillage/déverrouillage est implémentée nativement pour un maximum de fiabilité et de performance, avec une couverture de tests dédiée.
- ⚡ **Propulsé par [Nitro Modules](https://nitro.margelo.com/)** — une intégration native rapide et typée avec React Native.
- 📱 **Multiplateforme** — support iOS et Android.

## 🏗️ Architecture

Le projet est organisé en deux grandes parties :

- **`src/`** — l'interface TypeScript exposée aux applications React Native.
- **`native/core/`** — le cœur métier écrit en Rust, structuré autour de cas d'usage (*use cases*) indépendants de toute plateforme :
  - verrouillage de l'appareil,
  - octroi de temps avec validation de PIN et suivi du crédit de temps restant.

Cette séparation permet de garder la logique métier testable, robuste et réutilisable, indépendamment de la couche native iOS/Android.

## 📦 Installation

Le module nécessite `react-native-nitro-modules` comme dépendance, puisqu'il s'appuie sur Nitro Modules pour le pont natif.

## 🛠️ Développement

Le projet utilise Yarn workspaces avec une application d'exemple (`example/`) pour tester le module en conditions réelles, ainsi que Turbo pour l'orchestration des tâches.

Côté natif, le cœur Rust dispose de sa propre suite de tests (via `cargo nextest`), garantissant le bon comportement des cas d'usage indépendamment de l'intégration React Native.

## 🤝 Contribuer

- [Workflow de développement](CONTRIBUTING.md#development-workflow)
- [Envoyer une pull request](CONTRIBUTING.md#sending-a-pull-request)
- [Code de conduite](CODE_OF_CONDUCT.md)

## 📄 Licence

MIT

---

Réalisé avec [create-react-native-library](https://github.com/callstack/react-native-builder-bob)
