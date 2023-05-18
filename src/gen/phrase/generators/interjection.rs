mod list;

use crate::config;
use list::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub fn interjection() -> &'static str {
    interjections().choose(&mut thread_rng()).expect("expected an interjection to exist")
}

pub fn maybe_interjection() -> Option<&'static str> {
    thread_rng().gen_bool(config().interjection_chance).then(|| interjection())
}
