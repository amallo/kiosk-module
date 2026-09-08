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
impl PinValidator for SuccessfullPinValidator{
    fn validate(&self, _pin: u8) -> bool {
        return true
    }
}
