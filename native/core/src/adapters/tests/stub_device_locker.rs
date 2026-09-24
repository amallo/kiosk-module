#[cfg(test)]
use async_trait::async_trait;

#[cfg(test)]
use crate::adapters::device_locker::DeviceLocker;

#[cfg(test)]
use crate::adapters::device_locker::DeviceLockerError;


#[cfg(test)]
pub struct FailureDeviceLocker{}

#[cfg(test)]
impl FailureDeviceLocker{
  pub fn new()->Self{
    return FailureDeviceLocker{}
  }
}

#[cfg(test)]
#[async_trait]
impl DeviceLocker for FailureDeviceLocker{

  async fn schedule_lock(&self, _at: u64)->Result<(), DeviceLockerError>{
    return Err(DeviceLockerError::Unknown)
  }
  async fn unlock_now(&self)->Result<(), DeviceLockerError>{
    return Err(DeviceLockerError::Unknown)
  }
  async fn lock_now(&self)->Result<(), DeviceLockerError>{
    return Err(DeviceLockerError::Unknown)
  }
}
