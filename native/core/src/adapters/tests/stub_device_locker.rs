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
   async fn lock(&self)->Result<(), DeviceLockerError>{
    return Err(DeviceLockerError::Unknown)
  }
  async fn unlock(&self)->Result<(), DeviceLockerError>{
    return Err(DeviceLockerError::Unknown)
  }
}
