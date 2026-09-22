use async_trait::async_trait;
use kiosk_core::adapters::device_locker::{DeviceLocker, DeviceLockerError};

/// DeviceLocker no-op qui logue les appels (visible via `adb logcat`).
/// Permet de valider tout le pipeline JNI de bout en bout sans dépendre
/// de DevicePolicyManager/Device Owner. À remplacer par une implémentation
/// réelle si un verrouillage effectif de l'appareil est nécessaire.
pub struct LoggingDeviceLocker;

impl LoggingDeviceLocker {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DeviceLocker for LoggingDeviceLocker {
    async fn unlock_now(&self) -> Result<(), DeviceLockerError> {
        log::info!("[LoggingDeviceLocker] unlock_now called");
        Ok(())
    }

    async fn schedule_lock(&self, at: u64) -> Result<(), DeviceLockerError> {
        log::info!("[LoggingDeviceLocker] schedule_lock called (at={at})");
        Ok(())
    }
}
