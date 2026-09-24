use async_trait::async_trait;

/// Notifie un observateur (typiquement l'UI JS via un pont natif) de l'état de
/// verrouillage courant, décidé exclusivement par les use cases. Fire-and-forget :
/// une notification manquée ne doit jamais faire échouer le use case lui-même.
#[async_trait]
pub trait LockStateNotifier {
    async fn notify_locked(&self);
    async fn notify_unlocked(&self);
}
