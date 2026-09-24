use crate::{adapters::{clock::Clock, device_locker::DeviceLocker, lock_state_notifier::LockStateNotifier, time_credit_storage::{GrantedTimeCredit, TimeCreditStorage}}, usecases::errors::UseCaseError};
use std::sync::Arc;

pub struct EnforceTimeCreditUseCase<DL: DeviceLocker, TC: TimeCreditStorage, C: Clock, LSN: LockStateNotifier>{
  device_locker: Arc<DL>,
  time_credit_storage: Arc<TC>,
  clock: Arc<C>,
  lock_state_notifier: Arc<LSN>,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TimeCreditPermission {
    Granted,
    Denied
}

fn lock_failure<E>(_: E) -> UseCaseError { UseCaseError::LockDeviceFailure }

impl<DL, TC, C, LSN> EnforceTimeCreditUseCase<DL, TC, C, LSN> where DL: DeviceLocker, TC: TimeCreditStorage, C: Clock, LSN: LockStateNotifier{
  pub fn new(device_locker: Arc<DL>, time_credit_storage: Arc<TC>, clock: Arc<C>, lock_state_notifier: Arc<LSN>)-> Self{
    EnforceTimeCreditUseCase{device_locker, time_credit_storage, clock, lock_state_notifier}
  }

   pub async fn execute(&self)->Result<TimeCreditPermission, UseCaseError>{
      let now = self.clock.now();
      let granted = self.time_credit_storage.granted_until().await.map_err(lock_failure)?;

      match granted {
        GrantedTimeCredit::Until { time } if now < time => {
          self.device_locker.unlock_now().await.map_err(lock_failure)?;
          self.device_locker.schedule_lock(time).await.map_err(lock_failure)?;
          self.lock_state_notifier.notify_unlocked().await;
          Ok(TimeCreditPermission::Granted)
        }
        GrantedTimeCredit::Until { .. } | GrantedTimeCredit::Denied => {
          self.device_locker.lock_now().await.map_err(lock_failure)?;
          self.lock_state_notifier.notify_locked().await;
          Ok(TimeCreditPermission::Denied)
        }
      }
  }
}

#[cfg(test)]
mod tests {

use crate::adapters::{clock::{Clock, MockClock}, tests::{mock_time_credit_storage::MockTimeCreditStorage, spy_device_locker::SpyDeviceLocker, spy_lock_state_notifier::SpyLockStateNotifier}, time_credit_storage::{GrantedTimeCredit, TimeCreditStorage}};

use super::*;

    fn setup<DL: DeviceLocker, TC: TimeCreditStorage, C: Clock, LSN: LockStateNotifier>(device_locker: Arc<DL>, time_credit_storage: Arc<TC>, clock: Arc<C>, lock_state_notifier: Arc<LSN>) -> EnforceTimeCreditUseCase<DL, TC, C, LSN> {
       return EnforceTimeCreditUseCase::new( device_locker, time_credit_storage, clock, lock_state_notifier);
    }


    #[tokio::test]
     async fn it_unlocks_granted_device(){

      let time_credit_storage = Arc::new(MockTimeCreditStorage::new());
      let clock = Arc::new(MockClock::new(1788851260892));
      let device_locker = Arc::new(SpyDeviceLocker::new());
      let lock_state_notifier = Arc::new(SpyLockStateNotifier::new());
      time_credit_storage.granted_until_will_return(Ok(GrantedTimeCredit::Until { time: 1788851260892 + 12 }));


      let use_case = setup(Arc::clone(&device_locker), Arc::clone(&time_credit_storage), Arc::clone(&clock), Arc::clone(&lock_state_notifier));
      let result = use_case.execute().await;
      assert_eq!(result, Ok(TimeCreditPermission::Granted));
      assert_eq!(device_locker.unlock_now_was_called(), true);
      assert_eq!(device_locker.schedule_lock_was_called_with(1788851260892 + 12), true);
      assert_eq!(lock_state_notifier.notify_unlocked_was_called(), true);
      assert_eq!(lock_state_notifier.notify_locked_was_called(), false);
    }

    #[tokio::test]
    async fn it_locks_device_on_elapsed_time(){

      let time_credit_storage = Arc::new(MockTimeCreditStorage::new());
      let clock = Arc::new(MockClock::new(1788851260892));
      let device_locker = Arc::new(SpyDeviceLocker::new());
      let lock_state_notifier = Arc::new(SpyLockStateNotifier::new());
      time_credit_storage.granted_until_will_return(Ok(GrantedTimeCredit::Until { time: 1788851260892 - 12 }));


      let use_case = setup(Arc::clone(&device_locker), Arc::clone(&time_credit_storage), Arc::clone(&clock), Arc::clone(&lock_state_notifier));
      let result = use_case.execute().await;
      assert_eq!(result, Ok(TimeCreditPermission::Denied));
      assert_eq!(device_locker.lock_now_was_called(), true);
      assert_eq!(lock_state_notifier.notify_locked_was_called(), true);
      assert_eq!(lock_state_notifier.notify_unlocked_was_called(), false);
    }

}
