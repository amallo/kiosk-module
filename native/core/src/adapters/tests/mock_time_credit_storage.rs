#[cfg(test)]
use std::sync::{Arc, Mutex};

#[cfg(test)]
use async_trait::async_trait;

#[cfg(test)]
use crate::adapters::time_credit_storage::{GrantedTimeCredit, StorageError, TimeCredit, TimeCreditStorage};


#[cfg(test)]
pub struct MockTimeCreditStorage{
  grant_will_return: Arc<Mutex<Option<Result<(), StorageError>>>>,
  granted_until_will_return: Arc<Mutex<Option<Result<GrantedTimeCredit, StorageError>>>>,
}


#[cfg(test)]
impl MockTimeCreditStorage{
    pub fn new() -> Self {
        Self {
          grant_will_return: Arc::new(Mutex::new(None)),
          granted_until_will_return: Arc::new(Mutex::new(None)),
        }
    }
    pub fn granted_until_will_return(&self, expected: Result<GrantedTimeCredit, StorageError>){
      *self.granted_until_will_return.lock().unwrap()= Some(expected)
    }
}

#[cfg(test)]
#[async_trait]
impl TimeCreditStorage for MockTimeCreditStorage{
    async fn grant(&self, _credit: TimeCredit)->Result<(), StorageError> {
      return Ok(())
    }
    async fn granted_until(&self) -> Result<GrantedTimeCredit, StorageError>{
      let result= self.granted_until_will_return.lock().unwrap();
      return result.expect("should be a return value here");
    }

}
