use crate::arabic::alphabet;
use crate::arabic::alphabet::{Alphabet, Diacritic};
use crate::arabic::error::TasrifError;
use crate::arabic::word::Base;

mod madi;

// page 13
// TOOD: add other types of verb
#[derive(Debug, PartialEq)]
enum BaseType {
    TholathiMojarad,
    RobaeiMojarad,
    TholathiMazid,
    RobaeiMazid,
    // TODO: remove unkown type
    Unkown,
}

#[derive(PartialEq, Debug)]
enum Form {}

#[derive(PartialEq, Debug)]
enum Person {
    First,  // متکلم
    Second, // مخاطب
    Third,  // غائب
}

#[derive(PartialEq, Debug)]
enum Number {
    Singular, // مفرد
    Dual,     // مثنی
    Plural,   // جمع
}

#[derive(PartialEq, Debug)]
enum Tense {
    Madi,
    Mudari,
    Amr,
}

// https://en.wikipedia.org/wiki/Arabic_verbs
#[derive(Debug)]
pub struct Verb {
    base: Base,
    tense: Tense,
    base_type: BaseType,
    person: Person,
    number: Number,
}
