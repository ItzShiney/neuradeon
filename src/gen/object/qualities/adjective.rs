use crate::Case;
use crate::Cases;
use crate::Gender;
use crate::Genders;

pub struct Adjective(Genders<Cases<&'static str>>);

impl From<Genders<Cases<&'static str>>> for Adjective {
    fn from(forms: Genders<Cases<&'static str>>) -> Self {
        Self(forms)
    }
}

impl Adjective {
    pub fn form(&self, gender: Gender, case: Case) -> &'static str {
        self.0[gender as usize][case as usize]
    }
}
