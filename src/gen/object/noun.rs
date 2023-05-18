mod list;
mod prefixes;

use crate::config;
use crate::Case;
use crate::Gender;
use crate::NounPrefixRate;
use list::*;
use prefixes::*;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

fn raw_noun(gender: Gender) -> &'static Noun {
    if gender == Gender::Plural {
        nouns().choose(&mut thread_rng()).expect("expected a noun to exist")
    } else {
        nouns()
            .iter()
            .filter(|noun| noun.gender == gender)
            .choose(&mut thread_rng())
            .expect("expected a noun to match the gender")
    }
}

fn noun_prefix() -> &'static str {
    noun_prefixes().choose(&mut thread_rng()).expect("expected a prefix to exist")
}

pub fn noun(gender: Gender, case: Case) -> String {
    let noun = raw_noun(gender);

    let prefix_chance = match noun.prefix_rate {
        NounPrefixRate::Never => 0.,
        NounPrefixRate::Sometimes => config().noun_prefix_chance,
        NounPrefixRate::Always => 1.,
    };

    let noun = noun.form(gender.number(), case);

    if thread_rng().gen_bool(prefix_chance) {
        format!("{}{}", noun_prefix(), noun)
    } else {
        noun.into()
    }
}
