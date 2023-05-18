mod generators;

pub use generators::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn phrase() -> String {
    phrase_generators().choose(&mut thread_rng()).unwrap()()
}
