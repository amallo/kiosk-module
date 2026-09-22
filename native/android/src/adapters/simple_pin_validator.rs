use async_trait::async_trait;
use kiosk_core::adapters::pin_validator::PinValidator;

/// Validateur de PIN minimal : compare à une valeur fixe passée à la construction.
/// La provenance définitive du PIN (config statique, stockage chiffré, ...) est
/// hors scope de ce premier chantier.
pub struct SimplePinValidator {
    expected_pin: u8,
}

impl SimplePinValidator {
    pub fn new(expected_pin: u8) -> Self {
        Self { expected_pin }
    }
}

#[async_trait]
impl PinValidator for SimplePinValidator {
    async fn validate(&self, pin: u8) -> bool {
        pin == self.expected_pin
    }
}
