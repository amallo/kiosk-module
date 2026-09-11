#[cfg(test)]
use std::sync::{Arc, Mutex};

#[cfg(test)]
use async_trait::async_trait;

#[cfg(test)]
use crate::adapters::time_credit_storage::{TimeCreditStorage, StorageError, TimeCredit};


#[cfg(test)]
pub struct MockTimeCreditStorage{
  grant_params: Arc<Mutex<Option<TimeCredit>>>,
}


#[cfg(test)]
impl MockTimeCreditStorage{
    pub fn new() -> Self {
        Self { grant_params: Arc::new(Mutex::new(None)), }
    }
    pub fn grant_was_called_with(&self, expected: TimeCredit)-> bool{
      return  *self.grant_params.lock().unwrap() == Some(expected)
    }
}

#[cfg(test)]
#[async_trait]
impl TimeCreditStorage for MockTimeCreditStorage{
    async fn grant(&self, credit: TimeCredit)->Result<(), StorageError> {
      let mut data = self.grant_params.lock().unwrap();
      *data = Some(credit);
      return Ok(())
    }

}
