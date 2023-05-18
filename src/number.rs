#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Number {
    Singular,
    Plural,
}

pub type SgPl<T> = [T; 2];
