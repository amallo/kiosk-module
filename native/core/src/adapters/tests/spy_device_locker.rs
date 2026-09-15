#[cfg(test)]
use std::sync::{Arc, Mutex};

#[cfg(test)]
use async_trait::async_trait;

#[cfg(test)]
use crate::adapters::device_locker::DeviceLocker;

#[cfg(test)]
use crate::adapters::device_locker::DeviceLockerError;


#[cfg(test)]
pub struct SpyDeviceLocker{
  lock_was_called: Arc<Mutex<bool>>,
  unlock_was_called: Arc<Mutex<bool>>,
}


#[cfg(test)]
impl SpyDeviceLocker{
  pub fn new()->Self{
    return SpyDeviceLocker{
      lock_was_called: Arc::new(Mutex::new(false)),
      unlock_was_called: Arc::new(Mutex::new(false))
    }
  }
  pub fn lock_was_called(&self)->bool{
    return *self.lock_was_called.lock().unwrap();
  }
  pub fn unlock_was_called(&self)->bool{
    return *self.unlock_was_called.lock().unwrap();
  }
}

#[cfg(test)]
#[async_trait]
impl DeviceLocker for SpyDeviceLocker{
   async fn lock(&self)->Result<(), DeviceLockerError>{
    let mut was_called = self.lock_was_called.lock().unwrap();
    *was_called = true;
    return Ok(())
  }
  async fn unlock(&self)->Result<(), DeviceLockerError>{
    let mut was_called = self.unlock_was_called.lock().unwrap();
    *was_called = true;
    return Ok(())
  }
}
