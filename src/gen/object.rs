mod noun;
mod qualities;

use crate::Case;
use crate::Gender;
pub use noun::*;
pub use qualities::*;
use rand::random;

pub fn object(gender: Gender, case: Case) -> String {
    let noun = noun(gender, case).to_owned();
    let qualities = qualities(gender, case);

    if qualities.is_empty() {
        noun
    } else {
        if random() {
            format!("{noun} {qualities}")
        } else {
            format!("{qualities} {noun}")
        }
    }
}
