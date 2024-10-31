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
    pub fn new(reflector: Reflector, rotors: [Rotor; 3], plugboard: Plugboard<N>) -> Self {
        Self { rotors, reflector, plugboard }
    }

    pub fn encode<I>(&mut self, input: I) -> EnigmaAlphabet
    where
        I: Into<EnigmaAlphabet>,
    {
        // turn rotors on press
        for i in (0..3).rev() {
            if !self.rotors[i].turn() {
                break;
            }
        }

        let input: EnigmaAlphabet = input.into();
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

        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::historical_machines::enigma_m3::*;

    #[ignore]
    #[test]
    fn test_dummy1() {
        // rotor I:   EKMFLGDQVZNTOWYHXUSPAIBRCJ
        // rotor II:  AJDKSIRUXBLHWTMCQGZNPYFVOE
        // rotor III: BDFHJLCPRTXVZNYEIWGAKMUSQO

        let mut enigma_m3 = Enigma::new(
            reflector_b(),
            [rotor_i(0), rotor_ii(0), rotor_iii(0)],
            Plugboard::<0>::new([]),
        );

        let input = 'G';
        let output = enigma_m3.encode(input);
        assert_eq!(output, 'P'.into());

        let mut enigma_m3 = Enigma::new(
            reflector_b(),
            [rotor_i(0), rotor_ii(0), rotor_iii(0)],
            Plugboard::<0>::new([]),
        );

        let input = 'P';
        let output = enigma_m3.encode(input);
        assert_eq!(output, 'G'.into());
    }

    #[test]
    fn test_dummy2() {
        let mut enigma_m3 = Enigma::new(
            reflector_b(),
            [rotor_ii(25), rotor_ii(0), rotor_ii(0)],
            Plugboard::<0>::new([]),
        );

        // enigma_m3.rotors[2].set_ring(1);

        let input = 'A';
        let output = enigma_m3.encode(input);
        assert_eq!(output, 'Z'.into());
    }

    #[test]
    fn operation_barbarossa_1941() {
        let mut enigma = Enigma::new(
            reflector_b(),
            [rotor_ii(1), rotor_iv(20), rotor_v(11)],
            Plugboard::new([
                ('A'.into(), 'V'.into()),
                ('B'.into(), 'S'.into()),
                ('C'.into(), 'G'.into()),
                ('D'.into(), 'L'.into()),
                ('F'.into(), 'U'.into()),
                ('H'.into(), 'Z'.into()),
                ('I'.into(), 'N'.into()),
                ('K'.into(), 'M'.into()),
                ('O'.into(), 'W'.into()),
                ('R'.into(), 'X'.into()),
            ]),
        );
        enigma.rotors[0].set_ring(1);
        enigma.rotors[1].set_ring(20);
        enigma.rotors[2].set_ring(11);

        enigma.rotors[0].set('W');
        enigma.rotors[1].set('X');
        enigma.rotors[2].set('C');

        // check key message
        let message = "KCH";
        let mut res = Vec::new();
        for ch in message.chars() {
            res.push(enigma.encode(ch));
        }

        assert_eq!("BLA", res.iter().map(|&x| x.to_char()).collect::<String>());

        enigma.rotors[0].set_ring(1);
        enigma.rotors[1].set_ring(20);
        enigma.rotors[2].set_ring(11);

        enigma.rotors[0].set('B');
        enigma.rotors[1].set('L');
        enigma.rotors[2].set('A');

        let message = "EDPUD NRGYS ZRCXN UYTPO MRMBO FKTBZ REZKM LXLVE FGUEY SIOZV EQMIK UBPMM YLKLT TDEIS MDICA GYKUA CTCDO MOHWX MUUIA UBSTS LRNBZ SZWNR FXWFY SSXJZ VIJHI DISHP RKLKA YUPAD TXQSP INQMA TLPIF SVKDA SCTAC DPBOP VHJK".replace(" ", "");
        let mut res = Vec::new();
        for ch in message.replace(" ", "").chars() {
            res.push(enigma.encode(ch));
        }

        let decrupted = "AUFKL XABTE ILUNG XVONX KURTI NOWAX KURTI NOWAX NORDW ESTLX SEBEZ XSEBE ZXUAF FLIEG ERSTR ASZER IQTUN GXDUB ROWKI XDUBR OWKIX OPOTS CHKAX OPOTS CHKAX UMXEI NSAQT DREIN ULLXU HRANG ETRET ENXAN GRIFF XINFX RGTX".replace(" ", "");
        assert_eq!(
            decrupted.replace(" ", ""),
            res.iter().map(|&x| x.to_char()).collect::<String>()
        );

        // second part
        enigma.rotors[0].set('L');
        enigma.rotors[1].set('S');
        enigma.rotors[2].set('D');

        let message = "SFBWD NJUSE GQOBH KRTAR EEZMW KPPRB XOHDR OEQGB BGTQV PGVKB VVGBI MHUSZ YDAJQ IROAX SSSNR EHYGG RPISE ZBOVM QIEMM ZCYSG QDGRE RVBIL EKXYQ IRGIR QNRDN VRXCY YTNJR".replace(" ", "");
        let mut res = Vec::new();
        for ch in message.chars() {
            res.push(enigma.encode(ch));
        }

        let decrupted = "DREIG EHTLA NGSAM ABERS IQERV ORWAE RTSXE INSSI EBENN ULLSE QSXUH RXROE MXEIN SXINF RGTXD REIXA UFFLI EGERS TRASZ EMITA NFANG XEINS SEQSX KMXKM XOSTW XKAME NECXK".replace(" ", "");
        assert_eq!(decrupted, res.iter().map(|&x| x.to_char()).collect::<String>());

        // Reflector: B
        // Wheel order: II IV V
        // Ring positions: 02 21 12
        // Plug pairs: AV BS CG DL FU HZ IN KM OW RX

        // Message key: BLA
        // EDPUD NRGYS ZRCXN UYTPO MRMBO FKTBZ REZKM LXLVE FGUEY SIOZV EQMIK UBPMM YLKLT TDEIS MDICA GYKUA CTCDO MOHWX MUUIA UBSTS LRNBZ SZWNR FXWFY SSXJZ VIJHI DISHP RKLKA YUPAD TXQSP INQMA TLPIF SVKDA SCTAC DPBOP VHJK-
        // Decrypt: AUFKL XABTE ILUNG XVONX KURTI NOWAX KURTI NOWAX NORDW ESTLX SEBEZ XSEBE ZXUAF FLIEG ERSTR ASZER IQTUN GXDUB ROWKI XDUBR OWKIX OPOTS CHKAX OPOTS CHKAX UMXEI NSAQT DREIN ULLXU HRANG ETRET ENXAN GRIFF XINFX RGTX-

        // Message key: LSD
        // SFBWD NJUSE GQOBH KRTAR EEZMW KPPRB XOHDR OEQGB BGTQV PGVKB VVGBI MHUSZ YDAJQ IROAX SSSNR EHYGG RPISE ZBOVM QIEMM ZCYSG QDGRE RVBIL EKXYQ IRGIR QNRDN VRXCY YTNJR
        // Decrypt: DREIG EHTLA NGSAM ABERS IQERV ORWAE RTSXE INSSI EBENN ULLSE QSXUH RXROE MXEIN SXINF RGTXD REIXA UFFLI EGERS TRASZ EMITA NFANG XEINS SEQSX KMXKM XOSTW XKAME NECXK [truncated?]

        // German: Aufklärung abteilung von Kurtinowa nordwestlich Sebez[auf] Fliegerstraße in Richtung Dubrowki, Opotschka. Um 18:30 Uhr angetreten angriff.Infanterie Regiment 3 geht langsam aber sicher vorwärts. 17:06 Uhr röm eins InfanterieRegiment 3 auf Fliegerstraße mit Anfang 16km ostwärts Kamenec.
        // English: Reconnaissance division from Kurtinowa north-west of Sebezh on the flight corridor towards Dubrowki, Opochka.Attack begun at 18:30 hours.Infantry Regiment 3 goes slowly but surely forwards. 17:06 hours[Roman numeral I ?] Infantry Regiment 3 on the flight corridor starting 16 km east of Kamenec.
    }

    #[test]
    fn enigma_encode_multiple_steps() {
        let get_enigma = || {
            Enigma::new(
                reflector_b(),
                [rotor_i(0), rotor_ii(0), rotor_iii(0)],
                Plugboard::new([(0.into(), 1.into())]),
            )
        };

        let mut enima_m3 = get_enigma();

        const LEN: usize = 2000;
        let mut answer = Vec::new();
        for _ in 0..LEN {
            answer.push(dbg!(enima_m3.encode('A')));
        }

        let mut enigma_m3_backwards = get_enigma();
        let mut answer_backwards = Vec::new();

        for &x in answer.iter() {
            answer_backwards.push(enigma_m3_backwards.encode(x));
        }

        assert_eq!(vec![EnigmaAlphabet::from(0); LEN], answer_backwards);
    }
}
