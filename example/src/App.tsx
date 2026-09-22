import { useState } from 'react';
import { Button, Text, TextInput, View, StyleSheet } from 'react-native';
import { grantTime } from 'react-native-kiosk-module';

// Codes retournés par le pont natif (voir native/android/src/jni_bridge.rs)
const GRANT_CODE_LABELS: Record<number, string> = {
  0: 'OK',
  1: 'Accordé',
  2: 'Refusé',
  [-1]: 'Erreur de stockage du crédit de temps',
  [-2]: 'Code PIN invalide',
  [-3]: 'Erreur de verrouillage de l’appareil',
  [-100]: 'Erreur interne (panic)',
};

function describeGrantCode(code: number): string {
  return GRANT_CODE_LABELS[code] ?? `Code inconnu (${code})`;
}

export default function App() {
  const [durationSecs, setDurationSecs] = useState('60');
  const [pin, setPin] = useState('0');
  const [grantResult, setGrantResult] = useState<number | null>(null);

  return (
    <View style={styles.container}>
      <Text style={styles.label}>Durée du crédit de temps (secondes)</Text>
      <TextInput
        style={styles.input}
        keyboardType="number-pad"
        value={durationSecs}
        onChangeText={setDurationSecs}
        placeholder="Durée (secondes)"
      />
      <Text style={styles.label}>Code PIN</Text>
      <TextInput
        style={styles.input}
        keyboardType="number-pad"
        value={pin}
        onChangeText={setPin}
        placeholder="Code PIN"
      />
      <Button
        title="Grant time"
        onPress={() =>
          setGrantResult(grantTime(Number(durationSecs), Number(pin)))
        }
      />
      {grantResult !== null && (
        <Text>
          Grant code: {grantResult} ({describeGrantCode(grantResult)})
        </Text>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
  },
  label: {
    marginTop: 8,
    fontSize: 12,
    color: '#555',
  },
  input: {
    borderWidth: 1,
    borderColor: '#ccc',
    borderRadius: 4,
    width: 200,
    marginBottom: 8,
    paddingHorizontal: 8,
    paddingVertical: 4,
  },
});
