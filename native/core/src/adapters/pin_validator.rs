use async_trait::async_trait;


#[async_trait]
pub trait PinValidator{
  async fn validate(&self, pin: u8) -> bool;
}
