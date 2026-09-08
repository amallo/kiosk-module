#[cfg(test)]
use std::cell::RefCell;

#[cfg(test)]
use crate::adapters::time_credit_storage::{TimeCreditStorage, StorageError, TimeCredit};


#[cfg(test)]
pub struct FailureTimeCreditStorage{
  grant_failure: RefCell<Option<StorageError>>
}

#[cfg(test)]
impl FailureTimeCreditStorage {
  pub fn new() -> Self {
        Self {grant_failure: RefCell::new(None)}
    }
  pub fn grant_will_fail_with(&self, failure: StorageError){
    *self.grant_failure.borrow_mut() = Some(failure);
  }
}

#[cfg(test)]
impl TimeCreditStorage for FailureTimeCreditStorage{
    fn grant(&self, _: TimeCredit) -> Result<(), StorageError>{
        return Err(self.grant_failure.borrow().expect("FailureStorage: aucune erreur configurée, appelez save_credits_failure() d'abord"));
    }
}
