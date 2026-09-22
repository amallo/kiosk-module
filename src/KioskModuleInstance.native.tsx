import { NitroModules } from 'react-native-nitro-modules';
import type { KioskModule } from './KioskModule.nitro';

// Instance unique du HybridObject natif, partagée par toutes les fonctions
// exportées (grantTime, enforceTimeCredit, ...). Nitro construit une nouvelle
// instance à chaque appel de `createHybridObject` : la centraliser ici évite
// que deux modules distincts finissent chacun avec leur propre nativeHandle
// (et donc leur propre AppContext côté Rust).
export const KioskModuleHybridObject =
  NitroModules.createHybridObject<KioskModule>('KioskModule');
