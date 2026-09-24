#[cfg(test)]
use std::sync::{Arc, Mutex};

#[cfg(test)]
use async_trait::async_trait;

#[cfg(test)]
use crate::adapters::lock_state_notifier::LockStateNotifier;

#[cfg(test)]
pub struct SpyLockStateNotifier{
  notify_locked_was_called: Arc<Mutex<bool>>,
  notify_unlocked_was_called: Arc<Mutex<bool>>,
}

#[cfg(test)]
impl SpyLockStateNotifier{
  pub fn new()->Self{
    return SpyLockStateNotifier{
      notify_locked_was_called: Arc::new(Mutex::new(false)),
      notify_unlocked_was_called: Arc::new(Mutex::new(false)),
    }
  }
  pub fn notify_locked_was_called(&self)->bool{
    return *self.notify_locked_was_called.lock().unwrap();
  }
  pub fn notify_unlocked_was_called(&self)->bool{
    return *self.notify_unlocked_was_called.lock().unwrap();
  }
}

#[cfg(test)]
#[async_trait]
impl LockStateNotifier for SpyLockStateNotifier{

  async fn notify_locked(&self){
    let mut args = self.notify_locked_was_called.lock().unwrap();
    *args = true;
  }

  async fn notify_unlocked(&self){
    let mut args = self.notify_unlocked_was_called.lock().unwrap();
    *args = true;
  }
}
