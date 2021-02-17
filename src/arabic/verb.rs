use crate::arabic::alphabet;
use crate::arabic::alphabet::{Alphabet, Diacritic};
use crate::arabic::error::TasrifError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Verb {
    base: alphabet::Base,
}

impl FromStr for Verb {
    type Err = TasrifError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut base = Vec::new();
        let chars: Vec<_> = s.chars().collect();
        for ad in chars.chunks_exact(2) {
            let alphabet = alphabet::char_to_alphabet(ad[0]);
            let diacritic = alphabet::char_to_diacritic(ad[1]);
            if alphabet == Alphabet::None || diacritic == Diacritic::None {
                return Err(TasrifError::ParseError);
            }
            base.push((alphabet, diacritic));
        }

        Ok(Verb { base })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verb_from_str() {
        let verb = Verb::from_str("ضَرَبَ").unwrap();
        let zarab = vec![
            (Alphabet::Za, Diacritic::Fatha),
            (Alphabet::Ra, Diacritic::Fatha),
            (Alphabet::Ba, Diacritic::Fatha),
        ];
        assert_eq!(zarab, verb.base);
    }
}
