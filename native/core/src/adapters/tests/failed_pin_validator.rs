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
impl PinValidator for FailedPinValidator{
    fn validate(&self, _pin: u8) -> bool {
        return false
    }
}
