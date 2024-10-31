use super::wiring::Wiring;
use crate::core::alphabet::EnigmaAlphabet;
use crate::core::encoder::Encoder;

pub struct Reflector {
    wiring: Wiring,
}

impl Reflector {
    pub fn new(wiring: impl Into<Wiring>) -> Self {
        let wiring = wiring.into();
        assert!(
            ('A'..='Z').all(|x| wiring.ab_wire(wiring.ab_wire(x)) == x.into()),
            "reflector should be a mirror (e.i. permutation where cycles are only len 2) "
        );
        Self { wiring }
    }
}

impl Encoder for Reflector {
    type Letter = EnigmaAlphabet;

    fn encode<I>(&self, input: I) -> Self::Letter
    where
        I: Into<Self::Letter>,
    {
        self.wiring.ab_wire(input)
    }
}

impl From<&str> for Reflector {
    fn from(wiring: &str) -> Self {
        Self::new(wiring)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::historical_machines::enigma_m3::reflector_b;

    #[test]
    fn test_reflector() {
        let reflector = reflector_b();
        let encoded = reflector.encode('A');
        assert_eq!(encoded, 'Y'.into());
    }

    #[test]
    fn test_reflector_from_str() {
        let reflector: Reflector = "BADCFEHGJILKNMPORQTSVUXWZY".into();
        let encoded = reflector.encode(b'A');
        assert_eq!(encoded, 'B'.into());
    }

    #[test]
    #[should_panic]
    fn test_reflector_invalid() {
        let _ = Reflector::new("ABCDEFGHIJKLMNOPQRSTUVWXAZ");
    }
}
