use async_trait::async_trait;

#[derive(PartialEq)]
pub struct TimeCredit{
  pub start: u64,
  pub end: u64
}

#[derive(PartialEq, Clone, Copy)]
pub enum StorageError{
  WriteError
}

#[derive(PartialEq, Clone, Copy)]
pub enum GrantedTimeCredit{
  Until {time: u64},
  Denied
}


#[async_trait]
pub trait TimeCreditStorage{
  async fn grant(&self, credit: TimeCredit) -> Result<(), StorageError>;
  async fn granted_until(&self) -> Result<GrantedTimeCredit, StorageError>;
}

