export function subscribeLockState(
  _listener: (locked: boolean) => void
): () => void {
  throw new Error('subscribeLockState is not supported on this platform');
}
