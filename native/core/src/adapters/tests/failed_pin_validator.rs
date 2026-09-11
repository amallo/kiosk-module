
#[cfg(test)]
use async_trait::async_trait;

#[cfg(test)]
use crate::adapters::pin_validator::{PinValidator};


#[cfg(test)]
pub struct FailedPinValidator{}


#[cfg(test)]
impl FailedPinValidator{
  pub fn new()->Self{
    return FailedPinValidator{}
  }
}

#[cfg(test)]
#[async_trait]
impl PinValidator for FailedPinValidator{
    async fn validate(&self, _pin: u8) -> bool {
        false
    }
}
