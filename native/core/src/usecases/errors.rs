#[derive(PartialEq, Debug)]
pub enum UseCaseError {
    StorageFailure,
    PinValidationFaiure,
    LockDeviceFailure
}
