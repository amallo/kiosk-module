#[derive(PartialEq, Debug)]
pub enum UseCaseError {
    TimeCreditStorageFailure,
    PinValidationFailure,
    LockDeviceFailure
}
