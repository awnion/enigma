use super::wiring::Wiring;
use crate::core::alphabet::EnigmaAlphabet;
use crate::core::encoder::Encoder;

pub struct Reflector {
    wiring: Wiring,
}

impl Reflector {
    pub fn new(wiring: impl Into<Wiring>) -> Self {
        let wiring = wiring.into();
        Self { wiring }
    }
}

impl Encoder for Reflector {
    type Letter = EnigmaAlphabet;

    fn encode<I>(&self, input: I) -> Self::Letter
    where
        I: Into<Self::Letter>,
    {
        self.wiring.wire(input)
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

    #[test]
    fn test_reflector() {
        use EnigmaAlphabet::*;
        let reflector = Reflector::new([
            B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A,
        ]);
        let encoded = reflector.encode(A);
        assert_eq!(encoded, B);
    }

    #[test]
    fn test_reflector_from_str() {
        let reflector: Reflector = "YXWVUTSRQPONMLKJIHGFEDCBAZ".into();
        let encoded = reflector.encode(EnigmaAlphabet::A);
        assert_eq!(encoded, 'Y'.into());
    }

    #[test]
    #[should_panic]
    fn test_reflector_invalid() {
        use EnigmaAlphabet::*;
        let _ = Reflector::new([
            B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, A, A,
        ]);
    }
}
