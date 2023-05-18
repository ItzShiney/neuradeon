use crate::Case;
use crate::Cases;
use crate::Number;
use crate::SgPl;

pub struct NounForms(Cases<SgPl<&'static str>>);

impl From<Cases<SgPl<&'static str>>> for NounForms {
    fn from(forms: Cases<SgPl<&'static str>>) -> Self {
        Self(forms)
    }
}

impl NounForms {
    pub fn get(&self, number: Number, case: Case) -> &'static str {
        self.0[case as usize][number as usize]
    }
}
