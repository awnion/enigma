//! # Enigma M3
//!
//! ## German Navy (Kriegsmarine)
//!
//! Enigma M1, M2 and M3 were used by the German Navy (Kriegsmarine). As these
//! three models are functionally identical, they are usually referred to as M3.
//! They are compatible with Enigma I but have letters on the rotors rather than
//! numbers. The wiring of Enigma M3 is given in the table below. Rotors I to V
//! are identical to those of the Enigma I. The same is true for UKW B and C.
//! The three additional rotors (VI, VII and VIII) were used exclusively by the
//! Kriegsmarine. The machine is also compatible with Enigma M4 (when the 4th
//! rotor of the M4 is set to position 'A').
//!
//! |-------|----------------------------|----------|
//! | Rotor | ABCDEFGHIJKLMNOPQRSTUVWXYZ | Turnover |
//! |-------|----------------------------|----------|
//! | I     | EKMFLGDQVZNTOWYHXUSPAIBRCJ | Q        |
//! | II    | AJDKSIRUXBLHWTMCQGZNPYFVOE | E        |
//! | III   | BDFHJLCPRTXVZNYEIWGAKMUSQO | V        |
//! | IV    | ESOVPZJAYQUIRHXLNFTGKDCMWB | J        |
//! | V     | VZBRGITYUPSDNHLXAWMJQOFECK | Z        |
//! | VI    | JPGVOUMFYQBENHZRDKASXLICTW | ZM       |
//! | VII   | NZJHGRCXMYSWBOUFAIVLPEKQDT | ZM       |
//! | VIII  | FKQHTLXOCBJSPDZRAMEWNIUYGV | ZM       |
//! | UKW-B | YRUHQSLDPXNGOKMIEBFZCWVJAT |          |
//! | UKW-C | FVPJIAOYEDRZXWGCTKUQSBNMHL |          |
//! |-------|----------------------------|----------|

use crate::core::alphabet::EnigmaAlphabet;
use crate::core::parts::reflector::Reflector;
use crate::core::parts::rotor::Rotor;

pub const ROTOR_I: (&str, &str) = ("EKMFLGDQVZNTOWYHXUSPAIBRCJ", "Q");
pub const ROTOR_II: (&str, &str) = ("AJDKSIRUXBLHWTMCQGZNPYFVOE", "E");
pub const ROTOR_III: (&str, &str) = ("BDFHJLCPRTXVZNYEIWGAKMUSQO", "V");
pub const ROTOR_IV: (&str, &str) = ("ESOVPZJAYQUIRHXLNFTGKDCMWB", "J");
pub const ROTOR_V: (&str, &str) = ("VZBRGITYUPSDNHLXAWMJQOFECK", "Z");
pub const ROTOR_VI: (&str, &str) = ("JPGVOUMFYQBENHZRDKASXLICTW", "ZM");
pub const ROTOR_VII: (&str, &str) = ("NZJHGRCXMYSWBOUFAIVLPEKQDT", "ZM");
pub const ROTOR_VIII: (&str, &str) = ("FKQHTLXOCBJSPDZRAMEWNIUYGV", "ZM");
pub const UKW_B: &str = "YRUHQSLDPXNGOKMIEBFZCWVJAT";
pub const UKW_C: &str = "FVPJIAOYEDRZXWGCTKUQSBNMHL";

pub fn rotor_i(position: impl Into<EnigmaAlphabet>) -> Rotor {
    Rotor::new(position, ROTOR_I.1.chars(), ROTOR_I.0)
}

pub fn rotor_ii(position: impl Into<EnigmaAlphabet>) -> Rotor {
    Rotor::new(position, ROTOR_II.1, ROTOR_II.0)
}

pub fn rotor_iii(position: impl Into<EnigmaAlphabet>) -> Rotor {
    Rotor::new(position, ROTOR_III.1, ROTOR_III.0)
}

pub fn rotor_iv(position: impl Into<EnigmaAlphabet>) -> Rotor {
    Rotor::new(position, ROTOR_IV.1, ROTOR_IV.0)
}

pub fn rotor_v(position: impl Into<EnigmaAlphabet>) -> Rotor {
    Rotor::new(position, ROTOR_V.1, ROTOR_V.0)
}

pub fn rotor_vi(position: impl Into<EnigmaAlphabet>) -> Rotor {
    Rotor::new(position, ROTOR_VI.1, ROTOR_VI.0)
}

pub fn rotor_vii(position: impl Into<EnigmaAlphabet>) -> Rotor {
    Rotor::new(position, ROTOR_VII.1, ROTOR_VII.0)
}

pub fn rotor_viii(position: impl Into<EnigmaAlphabet>) -> Rotor {
    Rotor::new(position, ROTOR_VIII.1, ROTOR_VIII.0)
}

pub fn reflector_b() -> Reflector {
    Reflector::new(UKW_B)
}

pub fn reflector_c() -> Reflector {
    Reflector::new(UKW_C)
}
