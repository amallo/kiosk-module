use async_trait::async_trait;
use jni::objects::GlobalRef;
use jni::JavaVM;
use kiosk_core::adapters::time_credit_storage::{
    GrantedTimeCredit, StorageError, TimeCredit, TimeCreditStorage,
};

/// Sentinel renvoyé par TimeCreditStorageBridge.grantedUntil() quand aucun
/// crédit n'a jamais été accordé. Doit rester synchronisé avec
/// TimeCreditStorageBridge.NO_CREDIT côté Kotlin.
const NO_CREDIT_SENTINEL: i64 = -1;

/// Persiste le crédit de temps accordé via un rappel JNI vers un objet Kotlin
/// (`TimeCreditStorageBridge`) adossé à SharedPreferences, plutôt qu'un fichier
/// JSON sur disque. Rust conserve un `JavaVM` (pour attacher le thread appelant,
/// aujourd'hui toujours le thread unique du runtime tokio current-thread piloté
/// de façon synchrone depuis JNI) ainsi qu'une `GlobalRef` vers l'instance
/// Kotlin vivante (créée une fois dans nativeInit, valide pour toute la durée
/// de vie de l'AppContext).
pub struct SharedPreferencesTimeCreditStorage {
    vm: JavaVM,
    bridge: GlobalRef,
}

impl SharedPreferencesTimeCreditStorage {
    pub fn new(vm: JavaVM, bridge: GlobalRef) -> Self {
        Self { vm, bridge }
    }
}

#[async_trait]
impl TimeCreditStorage for SharedPreferencesTimeCreditStorage {
    async fn grant(&self, credit: TimeCredit) -> Result<(), StorageError> {
        let mut env = self
            .vm
            .attach_current_thread()
            .map_err(|_| StorageError::WriteError)?;

        let start = credit.start as i64;
        let end = credit.end as i64;

        let result = env.call_method(
            self.bridge.as_obj(),
            "grant",
            "(JJ)Z",
            &[start.into(), end.into()],
        );

        match result {
            Ok(value) => {
                let ok = value.z().map_err(|_| StorageError::WriteError)?;
                if ok {
                    Ok(())
                } else {
                    Err(StorageError::WriteError)
                }
            }
            Err(_) => {
                if env.exception_check().unwrap_or(false) {
                    let _ = env.exception_describe();
                    let _ = env.exception_clear();
                }
                Err(StorageError::WriteError)
            }
        }
    }

    async fn granted_until(&self) -> Result<GrantedTimeCredit, StorageError> {
        let mut env = self
            .vm
            .attach_current_thread()
            .map_err(|_| StorageError::WriteError)?;

        let result = env.call_method(self.bridge.as_obj(), "grantedUntil", "()J", &[]);

        match result {
            Ok(value) => {
                let raw = value.j().map_err(|_| StorageError::WriteError)?;
                if raw == NO_CREDIT_SENTINEL {
                    Ok(GrantedTimeCredit::Denied)
                } else {
                    Ok(GrantedTimeCredit::Until { time: raw as u64 })
                }
            }
            Err(_) => {
                if env.exception_check().unwrap_or(false) {
                    let _ = env.exception_describe();
                    let _ = env.exception_clear();
                }
                Err(StorageError::WriteError)
            }
        }
    }
}
