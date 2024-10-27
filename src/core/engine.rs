use super::parts::plugboard::Plugboard;
use super::parts::reflector::Reflector;
use super::parts::rotor::Rotor;

pub struct Enigma<const N: usize> {
    rotors: [Rotor; 3],
    reflector: Reflector,
    plugboard: Plugboard<N>,
}

impl<const N: usize> Enigma<N> {
    pub fn new(rotors: [Rotor; 3], reflector: Reflector, plugboard: Plugboard<N>) -> Self {
        Self { rotors, reflector, plugboard }
    }
}
