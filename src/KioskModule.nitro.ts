import type { HybridObject } from 'react-native-nitro-modules';

export interface KioskModule extends HybridObject<{
  ios: 'swift';
  android: 'kotlin';
}> {
  grantTime(durationSecs: number, pin: number): number;
  enforceTimeCredit(): number;
}
