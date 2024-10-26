pub struct Plugboard {
    pairs: [(char, char); 10],
}

pub struct Rotor {
    position: u8,
    wiring: [char; 26],
}

impl Rotor {
    pub fn new(position: u8, wiring: [char; 26]) -> Self {
        Self { position, wiring }
    }
}

struct Enigma {
    rotors: [Rotor; 3],
    reflector: [char; 26],
}

impl Enigma {
    pub fn new(rotors: [Rotor; 3], reflector: [char; 26]) -> Self {
        Self { rotors, reflector }
    }
}
