use super::BaseType;
use crate::arabic::alphabet::{Alphabet, Diacritic};
use crate::arabic::verb::{Verb, Person, Number, Tense};
use crate::arabic::word::Base;

pub fn create_madi(main_letters: [Alphabet; 3], middle_haraka: Diacritic) -> Verb {
    let mut base = Base::new();
    base.push((main_letters[0], Diacritic::Fatha));
    base.push((main_letters[1], middle_haraka));
    base.push((main_letters[2], Diacritic::Fatha));

    Verb {
        base,
	tense: Tense::Madi,
        base_type: BaseType::TholathiMojarad,
	person: Person::Third,
	number: Number::Singular
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let verb = create_madi([Alphabet::Za, Alphabet::Ra, Alphabet::Ba], Diacritic::Fatha);
        let zarab = vec![
            (Alphabet::Za, Diacritic::Fatha),
            (Alphabet::Ra, Diacritic::Fatha),
            (Alphabet::Ba, Diacritic::Fatha),
        ];
        assert_eq!(zarab, verb.base);
        assert_eq!(verb.base_type, BaseType::TholathiMojarad);
	assert_eq!(verb.person, Person::Third);
	assert_eq!(verb.number, Number::Singular);
	assert_eq!(verb.tense, Tense::Madi);
    }
}
