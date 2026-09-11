#[cfg(test)]
use async_trait::async_trait;

#[cfg(test)]
use crate::adapters::pin_validator::{PinValidator};


#[cfg(test)]
pub struct SuccessfullPinValidator{}


#[cfg(test)]
impl SuccessfullPinValidator{
  pub fn new()->Self{
    return SuccessfullPinValidator{}
  }
}

#[cfg(test)]
#[async_trait]
impl PinValidator for SuccessfullPinValidator{
    async fn validate(&self, _pin: u8) -> bool {
        true
    }
}
