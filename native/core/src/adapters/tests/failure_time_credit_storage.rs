#[cfg(test)]
use std::sync::{Arc, Mutex};


#[cfg(test)]
use async_trait::async_trait;

#[cfg(test)]
use crate::adapters::time_credit_storage::{TimeCreditStorage, StorageError, TimeCredit};


#[cfg(test)]
pub struct FailureTimeCreditStorage{
  grant_failure: Arc<Mutex<Option<StorageError>>>
}

#[cfg(test)]
impl FailureTimeCreditStorage {
  pub fn new() -> Self {
        Self {grant_failure: Arc::new(Mutex::new(None))}
  }
  pub fn grant_will_fail_with(&self, failure: StorageError){
    *self.grant_failure.lock().unwrap() = Some(failure);
  }
}

#[cfg(test)]
#[async_trait]
impl TimeCreditStorage for FailureTimeCreditStorage{
    async fn grant(&self, _: TimeCredit) -> Result<(), StorageError>{
        return Err(self.grant_failure.lock().unwrap().expect("FailureStorage: aucune erreur configurée, appelez save_credits_failure() d'abord"));
    }
}
