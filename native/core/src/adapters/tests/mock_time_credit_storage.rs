#[cfg(test)]
use std::cell::RefCell;

#[cfg(test)]
use crate::adapters::time_credit_storage::{TimeCreditStorage, StorageError, TimeCredit};


#[cfg(test)]

pub struct MockTimeCreditStorage{
  grant_params: RefCell<Option<TimeCredit>>
}


#[cfg(test)]
impl MockTimeCreditStorage{
    pub fn new() -> Self {
        Self { grant_params: RefCell::new(None) }
    }
    pub fn grant_was_called_with(&self, expected: TimeCredit)-> bool{
      return  *self.grant_params.borrow() == Some(expected)
    }
}

#[cfg(test)]
impl TimeCreditStorage for MockTimeCreditStorage{
    fn grant(&self, credit: TimeCredit)->Result<(), StorageError> {
      *self.grant_params.borrow_mut() = Some(credit);
      return Ok(())
    }

}
