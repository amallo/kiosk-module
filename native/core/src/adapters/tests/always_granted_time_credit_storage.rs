#[cfg(test)]
use std::sync::{Arc, Mutex};


#[cfg(test)]
use async_trait::async_trait;

use crate::adapters::time_credit_storage::GrantedTimeCredit;
#[cfg(test)]
use crate::adapters::time_credit_storage::{TimeCreditStorage, StorageError, TimeCredit};


#[cfg(test)]
pub struct AlwaysGrantedTimeCreditStorage{
  grant_failure: Arc<Mutex<Option<StorageError>>>
}

#[cfg(test)]
impl AlwaysGrantedTimeCreditStorage {
  pub fn new() -> Self {
        Self {grant_failure: Arc::new(Mutex::new(None))}
  }
  pub fn grant_will_fail_with(&self, failure: StorageError){
    *self.grant_failure.lock().unwrap() = Some(failure);
  }
}

#[cfg(test)]
#[async_trait]
impl TimeCreditStorage for AlwaysGrantedTimeCreditStorage{
    async fn grant(&self, _: TimeCredit) -> Result<(), StorageError>{
        return Ok(());
    }
    async fn granted_until(&self)-> Result<GrantedTimeCredit, StorageError>{
      return Ok(GrantedTimeCredit::Until { time: 12 });
    }
}
