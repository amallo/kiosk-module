#[cfg(test)]
use std::sync::{Arc, Mutex};

#[cfg(test)]
use async_trait::async_trait;

use crate::adapters::time_credit_storage::GrantedTimeCredit;
#[cfg(test)]
use crate::adapters::time_credit_storage::{TimeCreditStorage, StorageError, TimeCredit};


#[cfg(test)]
pub struct SpyTimeCreditStorage{
  grant_params: Arc<Mutex<Option<TimeCredit>>>,
}


#[cfg(test)]
impl SpyTimeCreditStorage{
    pub fn new() -> Self {
        Self {
          grant_params: Arc::new(Mutex::new(None)),
        }
    }
    pub fn grant_was_called_with(&self, expected: TimeCredit)-> bool{
      return  *self.grant_params.lock().unwrap() == Some(expected)
    }
}

#[cfg(test)]
#[async_trait]
impl TimeCreditStorage for SpyTimeCreditStorage{
    async fn grant(&self, credit: TimeCredit)->Result<(), StorageError> {
      let mut data = self.grant_params.lock().unwrap();
      *data = Some(credit);
      return Ok(())
    }
    async fn granted_until(&self)->Result<GrantedTimeCredit, StorageError> {
      return Ok(GrantedTimeCredit::Denied);
    }

}
