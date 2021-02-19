/// Arabic alphabets based on https://en.wikipedia.org/wiki/Arabic_alphabet
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Alphabet {
    Alif,
    Ba,
    Ta,
    Tha,
    Jim,
    Ha,
    Kha,
    Dal,
    Dhal,
    Ra,
    Zay,
    Sin,
    Shin,
    Sad,
    Dad,
    Ta2,
    Za,
    Ayn,
    Ghayn,
    Fa,
    Qaf,
    Kaf,
    Lam,
    Mim,
    Nun,
    Ha2,
    Waw,
    Ya,
    Hamzah,
    None,
}

/// Arabic main diacritics https://en.wikipedia.org/wiki/Arabic_diacritics
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Diacritic {
    Fatha,
    Kasra,
    Damma,
    Sukun,
    None,
}

pub fn char_to_alphabet(c: char) -> Alphabet {
    match c {
        'ض' => Alphabet::Za,
        'ر' => Alphabet::Ra,
        'ب' => Alphabet::Ba,
        _ => Alphabet::None,
    }
}

pub fn char_to_diacritic(c: char) -> Diacritic {
    match c {
        '\u{650}' => Diacritic::Kasra,
        '\u{64e}' => Diacritic::Fatha,
        '\u{64f}' => Diacritic::Damma,
        '\u{652}' => Diacritic::Sukun,
        _ => Diacritic::None,
    }
}
