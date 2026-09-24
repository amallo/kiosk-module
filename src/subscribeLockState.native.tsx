import { KioskModuleHybridObject } from './KioskModuleInstance.native';

export function subscribeLockState(
  listener: (locked: boolean) => void
): () => void {
  KioskModuleHybridObject.onLockStateChanged = listener;
  return () => {
    KioskModuleHybridObject.onLockStateChanged = () => {};
  };
}
