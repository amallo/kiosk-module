import { KioskModuleHybridObject } from './KioskModuleInstance.native';

export function grantTime(durationSecs: number, pin: number): number {
  return KioskModuleHybridObject.grantTime(durationSecs, pin);
}
