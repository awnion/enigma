use std::collections::HashSet;

use crate::core::alphabet::EnigmaAlphabet;

#[derive(Debug, Clone, Copy)]
pub struct Wiring {
    outputs: [EnigmaAlphabet; 26],
    inverts: [EnigmaAlphabet; 26],
}

impl Wiring {
    pub fn new(outputs: [EnigmaAlphabet; 26]) -> Self {
        let set: HashSet<EnigmaAlphabet> = outputs.into_iter().collect();
        assert!(set.len() == 26, "Wiring must contain 26 unique letters");

        let mut inverts = [0.into(); 26];
        for i in 0u8..26 {
            let l: usize = u8::from(outputs[i as usize]).into();
            inverts[l] = i.into();
        }

        Self { outputs, inverts }
    }

    pub(crate) fn ab_wire(&self, input: impl Into<EnigmaAlphabet>) -> EnigmaAlphabet {
        self.outputs[usize::from(input.into().to_u8())]
    }

    pub(crate) fn ba_wire(&self, input: impl Into<EnigmaAlphabet>) -> EnigmaAlphabet {
        self.inverts[usize::from(input.into().to_u8())]
    }
}

impl<T: Into<EnigmaAlphabet>> From<[T; 26]> for Wiring {
    fn from(outputs: [T; 26]) -> Self {
        let mut outputs_array = [0.into(); 26];
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
        let mut outputs_array = [0.into(); 26];
        for (output, output_array) in s.chars().zip(outputs_array.iter_mut()) {
            *output_array = output.into();
        }
        Self::new(outputs_array)
    }
}
