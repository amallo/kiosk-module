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
  lock_at_args: Arc<Mutex<u64>>,
  unlock_was_called: Arc<Mutex<bool>>,
}


#[cfg(test)]
impl SpyDeviceLocker{
  pub fn new()->Self{
    return SpyDeviceLocker{
      lock_at_args: Arc::new(Mutex::new(0)),
      unlock_was_called: Arc::new(Mutex::new(false))
    }
  }
  pub fn schedule_lock_was_called_with(&self, at: u64)->bool{
    return *self.lock_at_args.lock().unwrap() == at;
  }
  pub fn unlock_now_was_called(&self)->bool{
    return *self.unlock_was_called.lock().unwrap();
  }
}

#[cfg(test)]
#[async_trait]
impl DeviceLocker for SpyDeviceLocker{

  async fn schedule_lock(&self, at: u64)->Result<(), DeviceLockerError>{
    let mut args = self.lock_at_args.lock().unwrap();
    *args = at;
    return Ok(())
  }

  async fn unlock_now(&self)->Result<(), DeviceLockerError>{
    let mut args = self.unlock_was_called.lock().unwrap();
    *args = true;
    return Ok(())
  }
}
