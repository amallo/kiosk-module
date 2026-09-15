use crate::{adapters::{clock::Clock, device_locker::{DeviceLocker}, pin_validator::PinValidator, time_credit_storage::TimeCreditStorage}, usecases::{errors::UseCaseError}};
use std::{sync::Arc, time::Duration};
use crate::adapters::time_credit_storage::TimeCredit;

pub struct GrantTimeUseCase<S: TimeCreditStorage, C: Clock, PV: PinValidator, DL: DeviceLocker>{
  time_storage: Arc<S>,
  clock: Arc<C>,
  pin_validator: Arc<PV>,
  device_locker: Arc<DL>,
}

pub struct GrantTimeArgs {
  pub duration: Duration,
  pub pin: u8
}





impl<S,C, PV, DL> GrantTimeUseCase<S,C, PV, DL> where S: TimeCreditStorage, C: Clock, PV: PinValidator, DL: DeviceLocker{
  pub fn new(time_storage: Arc<S>, clock: Arc<C>, pin_validator: Arc<PV>, device_locker: Arc<DL>)-> Self{
    GrantTimeUseCase{time_storage, clock: clock, pin_validator: pin_validator, device_locker: device_locker}
  }

   pub async fn execute(&self, args: GrantTimeArgs)->Result<(), UseCaseError>{
    let now = self.clock.now();
    if !self.pin_validator.validate(args.pin).await{
      return  Err(UseCaseError::PinValidationFailure);
    }

    self.time_storage.grant(TimeCredit {
        start: now,
        end: now + args.duration.as_secs(),
    }).await.map_err(|_| UseCaseError::TimeCreditStorageFailure)?;

    self.device_locker.unlock().await
    .map_err(|_| UseCaseError::LockDeviceFailure)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
use crate::adapters::{clock::MockClock, tests::{failed_pin_validator::FailedPinValidator, failure_time_credit_storage::FailureTimeCreditStorage, mock_time_credit_storage::MockTimeCreditStorage, spy_device_locker::SpyDeviceLocker, successfull_pin_validator::SuccessfullPinValidator}, time_credit_storage::StorageError};

use super::*;

    fn setup<S: TimeCreditStorage, C:Clock, PV:PinValidator, DL: DeviceLocker>(time_credit_storage: Arc<S>, clock: Arc<C>, pin_validator: Arc<PV>, device_locker: Arc<DL>) -> GrantTimeUseCase<S, C, PV, DL> {
       return GrantTimeUseCase::new( time_credit_storage, clock, pin_validator, device_locker);
    }


    #[tokio::test]
     async fn it_successfully_grants_time(){

      let time_credit_storage = Arc::new(MockTimeCreditStorage::new());
      let clock = Arc::new(MockClock::new(1788851260892));
      let pin_validator = Arc::new(SuccessfullPinValidator::new());
      let device_locker = Arc::new(SpyDeviceLocker::new());

      let use_case = setup(Arc::clone(&time_credit_storage), Arc::clone(&clock), Arc::clone(&pin_validator), Arc::clone(&device_locker));
      let result = use_case.execute(GrantTimeArgs { duration: Duration::from_secs(12), pin:123 }).await;
      assert_eq!(result, Ok(()));
      assert_eq!(time_credit_storage.grant_was_called_with(TimeCredit{start : 1788851260892, end: 1788851260892+12}), true);
      assert_eq!(device_locker.unlock_was_called(), true)
    }

    #[tokio::test]
    async fn grant_storage_fails(){
      let failure_storage = FailureTimeCreditStorage::new();
      failure_storage.grant_will_fail_with(StorageError::WriteError);
      let time_credit_storage = Arc::new(failure_storage);

      let clock = Arc::new(MockClock::new(1788851260892));
      let pin_validator = Arc::new(SuccessfullPinValidator::new());
      let device_locker = Arc::new(SpyDeviceLocker::new());

      let use_case = setup(Arc::clone(&time_credit_storage), Arc::clone(&clock), Arc::clone(&pin_validator), Arc::clone(&device_locker));

      let result = use_case.execute(GrantTimeArgs { duration: Duration::from_secs(12), pin: 123 });
      assert_eq!(result.await, Err(UseCaseError::TimeCreditStorageFailure))
    }

     #[tokio::test]
    async fn pin_validation_fails() {
      let time_credit_storage = Arc::new(MockTimeCreditStorage::new());
      let clock = Arc::new(MockClock::new(1788851260892));
      let pin_validator = Arc::new(FailedPinValidator::new());
      let device_locker = Arc::new(SpyDeviceLocker::new());

      let use_case = setup(Arc::clone(&time_credit_storage), Arc::clone(&clock), Arc::clone(&pin_validator), Arc::clone(&device_locker));
      let result = use_case.execute(GrantTimeArgs { duration: Duration::from_secs(12), pin:123 });
      assert_eq!(result.await, Err(UseCaseError::PinValidationFailure))
    }
}
