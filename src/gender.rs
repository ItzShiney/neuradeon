use crate::Number;
use lazy_static::lazy_static;
use rand::distributions;
use rand::prelude::Distribution;
use rand::thread_rng;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
    Plural,
}

impl Gender {
    pub fn number(self) -> Number {
        match self {
            Gender::Masculine | Gender::Feminine | Gender::Neuter => Number::Singular,
            Gender::Plural => Number::Plural,
        }
    }
}

impl Gender {
    pub fn all() -> [Self; 4] {
        [Self::Masculine, Self::Feminine, Self::Neuter, Self::Plural]
    }

    pub fn choose() -> Self {
        lazy_static! {
            static ref WEIGHTS: distributions::WeightedIndex<f64> =
                distributions::WeightedIndex::new([1., 1., 1., 0.2]).unwrap();
        }

        Self::all()[WEIGHTS.sample(&mut thread_rng())]
    }
}

pub type Genders<T> = [T; 4];
