use async_trait::async_trait;


#[derive(PartialEq, Clone, Copy)]
pub enum DeviceLockerError{
  Unknown
}



#[async_trait]
pub trait DeviceLocker{
  async fn lock(&self) -> Result<(), DeviceLockerError>;
  async fn unlock(&self) -> Result<(), DeviceLockerError>;
}

