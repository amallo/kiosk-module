use crate::{adapters::device_locker::DeviceLocker, usecases::errors::UseCaseError};
use std::sync::Arc;

pub struct LockDeviceUseCase<DL: DeviceLocker>{
  device_locker: Arc<DL>
}



impl<DL> LockDeviceUseCase<DL> where DL: DeviceLocker{
  pub fn new(device_locker: Arc<DL>)-> Self{
    LockDeviceUseCase{device_locker}
  }

   pub async fn execute(&self)->Result<(), UseCaseError>{
    self.device_locker.lock().await
    .map_err(|_| UseCaseError::LockDeviceFailure)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {

use crate::adapters::tests::{spy_device_locker::SpyDeviceLocker, stub_device_locker::{ FailureDeviceLocker}};

use super::*;

    fn setup<DL: DeviceLocker>(device_locker: Arc<DL>) -> LockDeviceUseCase<DL> {
       return LockDeviceUseCase::new( device_locker);
    }


    #[tokio::test]
     async fn it_successfully_locks_device(){

      let device_locker = Arc::new(SpyDeviceLocker::new());

      let use_case = setup(Arc::clone(&device_locker));
      let result = use_case.execute().await;
      assert_eq!(result, Ok(()));
      assert_eq!(device_locker.lock_was_called(), true)
    }

    #[tokio::test]
    async fn it_fails_locks_device(){

      let device_locker = Arc::new(FailureDeviceLocker::new());

      let use_case = setup(Arc::clone(&device_locker));
      let result = use_case.execute();
      assert_eq!(result.await, Err(UseCaseError::LockDeviceFailure));
    }
}
