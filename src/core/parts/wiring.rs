use std::collections::HashSet;

use crate::core::alphabet::EnigmaAlphabet;

#[derive(Debug, Clone, Copy)]
pub struct Wiring {
    outputs: [EnigmaAlphabet; 26],
}

impl Wiring {
    pub fn new(outputs: [EnigmaAlphabet; 26]) -> Self {
        let set: HashSet<EnigmaAlphabet> = outputs.into_iter().collect();
        assert!(set.len() == 26, "Wiring must contain 26 unique letters");
        Self { outputs }
    }

    pub(crate) fn wire(&self, input: impl Into<EnigmaAlphabet>) -> EnigmaAlphabet {
        self.outputs[input.into() as usize]
    }

    pub(crate) fn unwire(&self, input: impl Into<EnigmaAlphabet>) -> EnigmaAlphabet {
        // TODO: optimize this
        let input = input.into();
        (0u8..26).find(|&i| self.wire(i) == input).expect("Letter not found in wiring").into()
    }
}

impl<T: Into<EnigmaAlphabet>> From<[T; 26]> for Wiring {
    fn from(outputs: [T; 26]) -> Self {
        let mut outputs_array = [EnigmaAlphabet::A; 26];
        for (output, output_array) in outputs.into_iter().zip(outputs_array.iter_mut()) {
            *output_array = output.into();
        }
        Self::new(outputs_array)
    }
}

impl From<&str> for Wiring {
    fn from(s: &str) -> Self {
        if s.len() != 26 {
            panic!("Wiring string must contain 26 characters");
        }
        let mut outputs_array = [EnigmaAlphabet::A; 26];
        for (output, output_array) in s.chars().zip(outputs_array.iter_mut()) {
            *output_array = output.into();
        }
        Self::new(outputs_array)
    }
}
