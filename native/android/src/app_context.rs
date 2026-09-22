use std::sync::Arc;

use kiosk_core::usecases::grant_time_use_case::GrantTimeUseCase;
use kiosk_core::usecases::lock_device_use_case::LockDeviceUseCase;

use crate::adapters::file_time_credit_storage::FileTimeCreditStorage;
use crate::adapters::logging_device_locker::LoggingDeviceLocker;
use crate::adapters::simple_pin_validator::SimplePinValidator;
use crate::adapters::system_clock::SystemClock;

/// PIN attendu par défaut. Placeholder pour ce premier chantier : la provenance
/// définitive (config statique, stockage chiffré, ...) est traitée séparément.
const DEFAULT_EXPECTED_PIN: u8 = 0;

type ConcreteLockUseCase = LockDeviceUseCase<LoggingDeviceLocker, FileTimeCreditStorage, SystemClock>;
type ConcreteGrantUseCase =
    GrantTimeUseCase<FileTimeCreditStorage, SystemClock, SimplePinValidator, LoggingDeviceLocker>;

/// Composition root : instancie une seule fois le runtime tokio et les use cases
/// (avec leurs adapters concrets) pour toute la durée de vie du handle natif
/// détenu côté Kotlin.
pub struct AppContext {
    pub runtime: tokio::runtime::Runtime,
    pub lock_use_case: ConcreteLockUseCase,
    pub grant_use_case: ConcreteGrantUseCase,
}

impl AppContext {
    pub fn new(storage_path: String) -> Self {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to build tokio runtime");

        let clock = Arc::new(SystemClock);
        let pin_validator = Arc::new(SimplePinValidator::new(DEFAULT_EXPECTED_PIN));
        let device_locker = Arc::new(LoggingDeviceLocker::new());
        let time_storage = Arc::new(FileTimeCreditStorage::new(storage_path));

        let lock_use_case = LockDeviceUseCase::new(
            Arc::clone(&device_locker),
            Arc::clone(&time_storage),
            Arc::clone(&clock),
        );
        let grant_use_case = GrantTimeUseCase::new(
            Arc::clone(&time_storage),
            Arc::clone(&clock),
            pin_validator,
            device_locker,
        );

        AppContext {
            runtime,
            lock_use_case,
            grant_use_case,
        }
    }
}
