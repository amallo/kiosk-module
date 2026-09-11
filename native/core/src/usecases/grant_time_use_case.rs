use crate::adapters::{clock::Clock, pin_validator::PinValidator, time_credit_storage::TimeCreditStorage};
use std::{sync::Arc, time::Duration};
use crate::adapters::time_credit_storage::TimeCredit;

pub struct GrantTimeUseCase<S: TimeCreditStorage, C: Clock, PV: PinValidator>{
  time_storage: Arc<S>,
  clock: Arc<C>,
  pin_validator: Arc<PV>
}

pub struct GrantTimeArgs {
  pub duration: Duration,
  pub pin: u8
}


#[derive(PartialEq, Debug)]
pub enum UseCaseError {
    StorageFailure,
    PinValidationFaiure
}



impl<S,C, PV> GrantTimeUseCase<S,C, PV> where S: TimeCreditStorage, C: Clock, PV: PinValidator{
  pub fn new(time_storage: Arc<S>, clock: Arc<C>, pin_validator: Arc<PV>)-> Self{
    GrantTimeUseCase{time_storage, clock: clock, pin_validator: pin_validator}
  }

   pub async fn execute(&self, args: GrantTimeArgs)->Result<(), UseCaseError>{
    let now = self.clock.now();
    if !self.pin_validator.validate(args.pin).await{
      return  Err(UseCaseError::PinValidationFaiure);
    }

    let save_result = self.time_storage.grant(TimeCredit {
        start: now,
        end: now + args.duration.as_secs(),
    }).await;
    match save_result {
      Ok(())=> Ok(()),
      Err(_)=>Err(UseCaseError::StorageFailure)
    }
  }
}

#[cfg(test)]
mod tests {
use crate::adapters::{clock::MockClock, tests::{failed_pin_validator::FailedPinValidator, failure_time_credit_storage::FailureTimeCreditStorage, mock_time_credit_storage::MockTimeCreditStorage, successfull_pin_validator::SuccessfullPinValidator}, time_credit_storage::StorageError};

use super::*;

    fn setup<S: TimeCreditStorage, C:Clock, PV:PinValidator>(time_credit_storage: Arc<S>, clock: Arc<C>, pin_validator: Arc<PV>) -> GrantTimeUseCase<S, C, PV> {
       return GrantTimeUseCase::new( time_credit_storage, clock, pin_validator);
    }


    #[tokio::test]
     async fn it_successfully_grants_time(){

      let time_credit_storage = Arc::new(MockTimeCreditStorage::new());
      let clock = Arc::new(MockClock::new(1788851260892));
      let pin_validator = Arc::new(SuccessfullPinValidator::new());

      let use_case = setup(Arc::clone(&time_credit_storage), Arc::clone(&clock), Arc::clone(&pin_validator));
      let result = use_case.execute(GrantTimeArgs { duration: Duration::from_secs(12), pin:123 });
      assert_eq!(result.await, Ok(()));
      assert_eq!(time_credit_storage.grant_was_called_with(TimeCredit{start : 1788851260892, end: 1788851260892+12}), true);

    }

    #[tokio::test]
    async fn grant_storage_fails(){
      let failure_storage = FailureTimeCreditStorage::new();
      failure_storage.grant_will_fail_with(StorageError::WriteError);
      let time_credit_storage = Arc::new(failure_storage);

      let clock = Arc::new(MockClock::new(1788851260892));
      let pin_validator = Arc::new(SuccessfullPinValidator::new());

      let use_case = setup(Arc::clone(&time_credit_storage), Arc::clone(&clock), Arc::clone(&pin_validator));

      let result = use_case.execute(GrantTimeArgs { duration: Duration::from_secs(12), pin: 123 });
      assert_eq!(result.await, Err(UseCaseError::StorageFailure))
    }

     #[tokio::test]
    async fn pin_validation_fails() {
      let time_credit_storage = Arc::new(MockTimeCreditStorage::new());
      let clock = Arc::new(MockClock::new(1788851260892));
      let pin_validator = Arc::new(FailedPinValidator::new());

      let use_case = setup(Arc::clone(&time_credit_storage), Arc::clone(&clock), Arc::clone(&pin_validator));
      let result = use_case.execute(GrantTimeArgs { duration: Duration::from_secs(12), pin:123 });
      assert_eq!(result.await, Err(UseCaseError::PinValidationFaiure))
    }
}
