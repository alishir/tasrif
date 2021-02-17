mod alphabet;
mod error;
mod verb;

enum Word {
    Verb(verb::Verb),
}
