mod forms;

use crate::Case;
use crate::Gender;
use crate::NounPrefixRate;
use crate::Number;
pub use forms::*;

pub struct Noun {
    pub gender: Gender,
    pub prefix_rate: NounPrefixRate,
    pub forms: NounForms,
}

impl Noun {
    pub fn form(&self, number: Number, case: Case) -> &'static str {
        self.forms.get(number, case)
    }
}
