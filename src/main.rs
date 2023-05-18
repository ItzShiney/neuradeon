mod case;
mod config;
pub mod gen;
mod gender;
mod noun_prefix_rate;
mod number;

pub use case::*;
pub use config::*;
pub use gender::*;
use lazy_static::lazy_static;
pub use noun_prefix_rate::*;
pub use number::*;
use std::path::Path;

pub fn config() -> &'static Config {
    lazy_static! {
        static ref CONFIG: Config = Config::load_or_create(Path::new("neuradeon.yaml"));
    }

    &CONFIG
}

fn main() {
    for _ in 1..=config().items_count {
        let res = gen::phrase();

        if !config().filter.passes(&res) {
            continue;
        }

        println!("{}", res);
    }

    if config().pause_after {
        loop {}
    }
}
