import { NitroModules } from 'react-native-nitro-modules';
import type { KioskModule } from './KioskModule.nitro';

const KioskModuleHybridObject =
  NitroModules.createHybridObject<KioskModule>('KioskModule');

export function multiply(a: number, b: number): number {
  return KioskModuleHybridObject.multiply(a, b);
}
