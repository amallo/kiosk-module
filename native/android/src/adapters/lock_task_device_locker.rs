use async_trait::async_trait;
use jni::objects::{GlobalRef, JValue};
use jni::JavaVM;
use kiosk_core::adapters::device_locker::{DeviceLocker, DeviceLockerError};
use kiosk_core::adapters::lock_state_notifier::LockStateNotifier;

/// Verrouille/déverrouille l'appareil via un rappel JNI vers un objet Kotlin
/// (`LockTaskBridge`) qui pilote l'Android Lock Task Mode (`startLockTask`/
/// `stopLockTask`) et l'`AlarmManager` (verrouillage différé), sans nécessiter
/// de Device Owner.
///
/// Trois primitives mécaniques, sans aucune logique ni comparaison : le choix
/// entre verrouiller immédiatement (`lock_now`) et programmer une alarme
/// (`schedule_lock`) appartient exclusivement à `EnforceTimeCreditUseCase`
/// (voir ARCHITECTURE.md).
pub struct LockTaskDeviceLocker {
    vm: JavaVM,
    bridge: GlobalRef,
}

impl LockTaskDeviceLocker {
    pub fn new(vm: JavaVM, bridge: GlobalRef) -> Self {
        Self { vm, bridge }
    }

    fn call_bool(&self, method: &str, sig: &str, args: &[JValue]) -> Result<(), DeviceLockerError> {
        let mut env = self
            .vm
            .attach_current_thread()
            .map_err(|_| DeviceLockerError::Unknown)?;

        let result = env.call_method(self.bridge.as_obj(), method, sig, args);

        if let Ok(value) = result
            && value.z().unwrap_or(false)
        {
            return Ok(());
        }

        if env.exception_check().unwrap_or(false) {
            let _ = env.exception_describe();
            let _ = env.exception_clear();
        }
        Err(DeviceLockerError::Unknown)
    }
}

#[async_trait]
impl DeviceLocker for LockTaskDeviceLocker {
    async fn unlock_now(&self) -> Result<(), DeviceLockerError> {
        self.call_bool("unlockNow", "()Z", &[])
    }

    async fn lock_now(&self) -> Result<(), DeviceLockerError> {
        self.call_bool("lockNow", "()Z", &[])
    }

    async fn schedule_lock(&self, at: u64) -> Result<(), DeviceLockerError> {
        let at = at as i64;
        self.call_bool("scheduleLock", "(J)Z", &[at.into()])
    }
}

#[async_trait]
impl LockStateNotifier for LockTaskDeviceLocker {
    async fn notify_locked(&self) {
        let _ = self.call_bool("notifyLocked", "()Z", &[]);
    }

    async fn notify_unlocked(&self) {
        let _ = self.call_bool("notifyUnlocked", "()Z", &[]);
    }
}
