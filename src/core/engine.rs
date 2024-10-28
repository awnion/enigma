use super::alphabet::EnigmaAlphabet;
use super::decoder::Decoder;
use super::encoder::Encoder;
use super::parts::plugboard::Plugboard;
use super::parts::reflector::Reflector;
use super::parts::rotor::Rotor;

pub struct Enigma<const N: usize> {
    rotors: [Rotor; 3],
    reflector: Reflector,
    plugboard: Plugboard<N>,
}

impl<const N: usize> Enigma<N> {
    pub fn new<R1, R2, P>(rotors: [R1; 3], reflector: R2, plugboard: P) -> Self
    where
        R1: Into<Rotor> + Copy,
        R2: Into<Reflector>,
        P: Into<Plugboard<N>>,
    {
        let rotors = [rotors[0].into(), rotors[1].into(), rotors[2].into()];
        let reflector = reflector.into();
        let plugboard = plugboard.into();
        Self { rotors, reflector, plugboard }
    }

    pub fn encode<I>(&mut self, input: I) -> EnigmaAlphabet
    where
        I: Into<EnigmaAlphabet>,
    {
        let input = self.plugboard.encode(input);
        let input = self.rotors[2].encode(input);
        let input = self.rotors[1].encode(input);
        let input = self.rotors[0].encode(input);
        let input = self.reflector.encode(input);
        // backwards
        let input = self.rotors[0].decode(input);
        let input = self.rotors[1].decode(input);
        let input = self.rotors[2].decode(input);
        let input = self.plugboard.encode(input);

        // turn rotors on press
        for i in 0..3 {
            if !self.rotors[i].turn() {
                break;
            }
        }

        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::historical_parts::reflectors::REFLECTOR_B;
    use crate::historical_parts::rotors::ROTOR_I;
    use crate::historical_parts::rotors::ROTOR_II;
    use crate::historical_parts::rotors::ROTOR_III;

    #[test]
    fn dummy_enigma() {
        // rotor I:   EKMFLGDQVZNTOWYHXUSPAIBRCJ
        // rotor II:  AJDKSIRUXBLHWTMCQGZNPYFVOE
        // rotor III: BDFHJLCPRTXVZNYEIWGAKMUSQO

        let mut enigma_m3 = Enigma::new(
            [
                Rotor::new(0, 'R', ROTOR_I),
                Rotor::new(0, 'F', ROTOR_II),
                Rotor::new(0, 'W', ROTOR_III),
            ],
            Reflector::new(REFLECTOR_B),
            Plugboard::<0>::new([]),
        );

        let input = 'G';
        let output = enigma_m3.encode(input);
        assert_eq!(output, 'P'.into());

        let mut enigma_m3 = Enigma::new(
            [
                Rotor::new(0, 'R', ROTOR_I),
                Rotor::new(0, 'F', ROTOR_II),
                Rotor::new(0, 'W', ROTOR_III),
            ],
            Reflector::new(REFLECTOR_B),
            Plugboard::<0>::new([]),
        );

        let input = 'P';
        let output = enigma_m3.encode(input);
        assert_eq!(output, 'G'.into());
    }
}
