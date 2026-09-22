import { KioskModuleHybridObject } from './KioskModuleInstance.native';

export function enforceTimeCredit(): number {
  return KioskModuleHybridObject.enforceTimeCredit();
}
