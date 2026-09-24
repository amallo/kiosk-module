import { useEffect, useState } from 'react';
import { Button, Text, TextInput, View, StyleSheet } from 'react-native';
import {
  enforceTimeCredit,
  grantTime,
  subscribeLockState,
} from 'react-native-kiosk-module';

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

// Voir native/android/src/jni_bridge.rs : enforceTimeCredit() retourne
// RESULT_GRANTED (1) quand le crédit en cours est encore valide ; tout le
// reste (dénié, erreur, panic) est traité comme verrouillé par défaut
// (fail-safe). grantTime(), lui, retourne RESULT_OK (0) sur succès (c'est un
// Result<(), _> côté Rust, pas un TimeCreditPermission) — les deux codes ne
// sont pas interchangeables.
const RESULT_GRANTED = 1;
const RESULT_OK = 0;

export default function App() {
  const [durationSecs, setDurationSecs] = useState('10');
  const [pin, setPin] = useState('0');
  const [grantResult, setGrantResult] = useState<number | null>(null);
  // null tant qu'on n'a pas encore de réponse : on reste verrouillé par défaut
  // pendant l'évaluation initiale (fail-safe).
  const [locked, setLocked] = useState(true);

  // Les appels JS (enforceTimeCredit, grantTime) sont synchrones : leur valeur
  // de retour suffit à dériver `locked` directement, pas besoin d'attendre un
  // event pour ces cas-là. Le seul cas qu'aucun retour de fonction ne peut
  // couvrir est l'alarme qui verrouille/déverrouille pendant que l'app est déjà
  // au premier plan (aucun appel JS en cours à ce moment-là) : c'est le seul
  // rôle de `subscribeLockState`, qui reçoit un push explicite décidé côté
  // Rust (EnforceTimeCreditUseCase/GrantTimeUseCase), jamais un polling JS.
  const evaluate = () => {
    const result = enforceTimeCredit();
    console.log('enforceTimeCredit', result);
    setLocked(result !== RESULT_GRANTED);
  };

  useEffect(() => {
    evaluate();
    return subscribeLockState(setLocked);
  }, []);

  const handleGrantTime = () => {
    const result = grantTime(Number(durationSecs), Number(pin));
    setGrantResult(result);
    if (result === RESULT_OK) {
      setLocked(false);
    }
  };

  return (
    <View style={styles.container}>
      <Text style={styles.label}>Code PIN</Text>
      <TextInput
        style={styles.input}
        keyboardType="number-pad"
        value={pin}
        onChangeText={setPin}
        placeholder="Code PIN"
      />
      <Text style={styles.label}>Durée du crédit de temps (secondes)</Text>
      <TextInput
        style={styles.input}
        keyboardType="number-pad"
        value={durationSecs}
        onChangeText={setDurationSecs}
        placeholder="Durée (secondes)"
      />
      {locked ? (
        <>
          <Text style={styles.lockedLabel}>Lock</Text>
          <Button title="Unlock" onPress={handleGrantTime} />
        </>
      ) : (
        <Button title="Grant time" onPress={handleGrantTime} />
      )}
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
  lockedLabel: {
    marginBottom: 8,
    fontSize: 20,
    fontWeight: 'bold',
    color: '#c00',
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
