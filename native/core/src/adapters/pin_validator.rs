
pub trait PinValidator{
  fn validate(&self, pin: u8) -> bool;
}
