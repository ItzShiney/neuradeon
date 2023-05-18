mod adjective;
mod adjectives;
mod amplifiers;

use crate::config;
use crate::Case;
use crate::Gender;
use adjective::*;
use amplifiers::*;
use itertools::Itertools;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

fn adjectives(count: usize) -> Vec<&'static Adjective> {
    adjectives::adjectives().choose_multiple(&mut thread_rng(), count).collect()
}

pub fn adjective(gender: Gender, case: Case) -> &'static str {
    adjectives(1)
        .choose(&mut thread_rng())
        .expect("expected an adjective to exist")
        .form(gender, case)
}

fn amplifier() -> &'static str {
    amplifiers().choose(&mut thread_rng()).expect("expected an amplifier to exist")
}

fn maybe_add_amplifier(adjective: &'static str) -> String {
    if thread_rng().gen_bool(config().amplifier_chance) {
        format!("{} {}", amplifier(), adjective)
    } else {
        adjective.into()
    }
}

pub fn qualities(gender: Gender, case: Case) -> String {
    let count = config()
        .qualities_range
        .clone()
        .choose(&mut thread_rng())
        .expect("qualities range was empty");

    adjectives(count)
        .into_iter()
        .map(|adjective| maybe_add_amplifier(adjective.form(gender, case)))
        .join(" ")
}
