#[derive(PartialEq)]
pub struct TimeCredit{
  pub start: u64,
  pub end: u64
}

#[derive(PartialEq, Clone, Copy)]
pub enum StorageError{
  WriteError
}
pub trait TimeCreditStorage{
  fn grant(&self, credit: TimeCredit) -> Result<(), StorageError>;
}

