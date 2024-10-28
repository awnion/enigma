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
        // TODO: Copy is not necessary here
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
        let input = input.into();
        let x = self.plugboard.encode(input);
        let x = self.rotors[2].encode(x);
        let x = self.rotors[1].encode(x);
        let x = self.rotors[0].encode(x);
        let x = self.reflector.encode(x);
        // backwards
        let x = self.rotors[0].decode(x);
        let x = self.rotors[1].decode(x);
        let x = self.rotors[2].decode(x);
        let x = self.plugboard.encode(x);

        assert_eq!(
            input,
            {
                let y = self.plugboard.encode(x);
                let y = self.rotors[2].encode(y);
                let y = self.rotors[1].encode(y);
                let y = self.rotors[0].encode(y);
                let y = self.reflector.encode(y);
                let y = self.rotors[0].decode(y);
                let y = self.rotors[1].decode(y);
                let y = self.rotors[2].decode(y);
                self.plugboard.encode(y)
            },
            "encode fail"
        );

        // turn rotors on press
        for i in 0..3 {
            if !self.rotors[i].turn() {
                break;
            }
        }

        x
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

    #[test]
    fn enigma_encode_multiple_steps() {
        let get_enigma = || {
            Enigma::new(
                [
                    Rotor::new(0, 'R', ROTOR_I),
                    Rotor::new(0, 'F', ROTOR_II),
                    Rotor::new(0, 'W', ROTOR_III),
                ],
                Reflector::new(REFLECTOR_B),
                Plugboard::new([(EnigmaAlphabet::A, EnigmaAlphabet::B)]),
            )
        };

        let mut enima_m3 = get_enigma();

        const LEN: usize = 2000;
        let mut answer = Vec::new();
        for _ in 0..LEN {
            answer.push(enima_m3.encode('A'));
        }

        let mut enigma_m3_backwards = get_enigma();
        let mut answer_backwards = Vec::new();

        for &x in answer.iter() {
            answer_backwards.push(enigma_m3_backwards.encode(x));
        }

        assert_eq!(vec![EnigmaAlphabet::A; LEN], answer_backwards);
    }
}
