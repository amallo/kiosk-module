# react-native-kiosk-module

Module React Native pour transformer un appareil en borne kiosque : verrouillage de l'écran et déverrouillage temporisé protégé par code PIN.

Un appareil (tablette, borne, terminal partagé...) reste verrouillé par défaut. On lui accorde un crédit de temps déverrouillé après validation d'un code PIN, puis il se reverrouille automatiquement une fois le temps écoulé.

## Fonctionnalités

- Verrouillage de l'appareil.
- Octroi de temps avec vérification d'un code PIN.
- Cœur métier écrit en Rust, avec sa propre suite de tests.
- Pont natif via [Nitro Modules](https://nitro.margelo.com/).
- iOS et Android.

## Architecture

Le projet est organisé en deux grandes parties :

- **`src/`** — l'interface TypeScript exposée aux applications React Native.
- **`native/core/`** — le cœur métier écrit en Rust, structuré autour de cas d'usage (*use cases*) indépendants de toute plateforme :
  - verrouillage de l'appareil,
  - octroi de temps avec validation de PIN et suivi du crédit de temps restant.

Cette séparation garde la logique métier testable et indépendante de la couche native iOS/Android.

## Installation

Le module nécessite `react-native-nitro-modules` comme dépendance, puisqu'il s'appuie sur Nitro Modules pour le pont natif.

## Développement

Le projet utilise Yarn workspaces avec une application d'exemple (`example/`) pour tester le module en conditions réelles, ainsi que Turbo pour l'orchestration des tâches.

Côté natif, le cœur Rust dispose de sa propre suite de tests (via `cargo nextest`), garantissant le bon comportement des cas d'usage indépendamment de l'intégration React Native.

Pour builder l'app example de bout en bout (y compris le crate Rust) ou ajouter un nouveau use case natif exposé en TS, voir le [guide de développement natif](docs/NATIVE_DEVELOPMENT.md).

## Contribuer

- [Workflow de développement](CONTRIBUTING.md#development-workflow)
- [Envoyer une pull request](CONTRIBUTING.md#sending-a-pull-request)
- [Code de conduite](CODE_OF_CONDUCT.md)

## Licence

MIT

---

Réalisé avec [create-react-native-library](https://github.com/callstack/react-native-builder-bob)
