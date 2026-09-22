use kiosk_core::usecases::errors::UseCaseError;
use kiosk_core::usecases::lock_device_use_case::TimeCreditPermission;
use jni::sys::jint;

// Codes de retour stables consommés côté Kotlin. On mappe sur des entiers plutôt
// que des exceptions Java pour rester simple côté binding.
pub const RESULT_OK: jint = 0;
pub const RESULT_GRANTED: jint = 1;
pub const RESULT_DENIED: jint = 2;
pub const ERR_TIME_CREDIT_STORAGE: jint = -1;
pub const ERR_PIN_VALIDATION: jint = -2;
pub const ERR_LOCK_DEVICE: jint = -3;
pub const ERR_INTERNAL_PANIC: jint = -100;

pub fn map_lock_result(result: Result<TimeCreditPermission, UseCaseError>) -> jint {
    match result {
        Ok(TimeCreditPermission::Granted) => RESULT_GRANTED,
        Ok(TimeCreditPermission::Denied) => RESULT_DENIED,
        Err(e) => map_use_case_error_code(e),
    }
}

pub fn map_use_case_result(result: Result<(), UseCaseError>) -> jint {
    match result {
        Ok(()) => RESULT_OK,
        Err(e) => map_use_case_error_code(e),
    }
}

fn map_use_case_error_code(error: UseCaseError) -> jint {
    match error {
        UseCaseError::TimeCreditStorageFailure => ERR_TIME_CREDIT_STORAGE,
        UseCaseError::PinValidationFailure => ERR_PIN_VALIDATION,
        UseCaseError::LockDeviceFailure => ERR_LOCK_DEVICE,
    }
}
