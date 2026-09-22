import { NitroModules } from 'react-native-nitro-modules';
import type { KioskModule } from './KioskModule.nitro';

const KioskModuleHybridObject =
  NitroModules.createHybridObject<KioskModule>('KioskModule');

export function grantTime(durationSecs: number, pin: number): number {
  return KioskModuleHybridObject.grantTime(durationSecs, pin);
}
