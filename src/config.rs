mod filter;

pub use filter::*;
use serde::Deserialize;
use serde::Serialize;
use serde_yaml as yaml;
use std::fs;
use std::ops::RangeInclusive;
use std::path::Path;
use yaml::Mapping;
use yaml::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub items_count: usize,
    pub filter: Filter,

    pub plural_weight: f64,
    pub noun_prefix_chance: f64,
    pub amplifier_chance: f64,
    pub interjection_chance: f64,
    pub qualities_range: RangeInclusive<usize>,

    pub completely_out_of_context_phrases: bool,
    pub self_deprecation: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            items_count: 30,
            filter: Filter::default(),

            plural_weight: 0.3,
            noun_prefix_chance: 0.7,
            amplifier_chance: 0.1,
            interjection_chance: 0.25,
            qualities_range: 0..=2,

            completely_out_of_context_phrases: true,
            self_deprecation: true,
        }
    }
}

impl Config {
    pub fn load_or_create(path: &Path) -> Self {
        create_if_missing(path);

        let mapping = yaml::from_reader::<_, Mapping>(fs::File::open(path).unwrap())
            .expect("failed to parse config");

        let res = yaml::from_value::<Self>(yaml::Value::Mapping(mapping.clone()))
            .expect("failed to parse config");

        let new_mapping = into_mapping(yaml::to_value(&res).unwrap());
        let missing_values = missing_values(new_mapping, &mapping);

        append_values(path, missing_values);

        res
    }
}

fn create_if_missing(path: &Path) {
    if !Path::exists(path) {
        fs::write(path, "").expect("failed to create config");
    }
}

fn into_mapping(value: Value) -> Mapping {
    let Value::Mapping(mapping) = value else { unreachable!() };
    mapping
}

fn missing_values(mut new_mapping: Mapping, old_mapping: &Mapping) -> Mapping {
    new_mapping.retain(|key, _| !old_mapping.contains_key(key));
    new_mapping
}

fn append_values(path: &Path, values: Mapping) {
    if !values.is_empty() {
        let file = fs::OpenOptions::new().append(true).open(path).unwrap();

        yaml::to_writer(file, &values).expect("failed to update config");
    }
}
