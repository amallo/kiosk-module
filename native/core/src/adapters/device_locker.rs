use async_trait::async_trait;


#[derive(PartialEq, Clone, Copy)]
pub enum DeviceLockerError{
  Unknown
}



#[async_trait]
pub trait DeviceLocker{
  async fn unlock_now(&self) -> Result<(), DeviceLockerError>;
  async fn lock_now(&self) -> Result<(), DeviceLockerError>;
  async fn schedule_lock(&self, at: u64) -> Result<(), DeviceLockerError>;
}

