pub trait Clock{
  fn now(&self)-> u64;
}

#[cfg(test)]
pub struct MockClock{
  now_value: u64
}

#[cfg(test)]
impl MockClock{
    pub fn new(now: u64) -> Self {
        Self { now_value: now }
    }
}

#[cfg(test)]
impl Clock for MockClock{
    fn now(&self)-> u64 {
        self.now_value
    }
}
