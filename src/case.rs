#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Case {
    /// Именительный
    Nom,
    /// Родительный
    Gen,
    /// Дательный
    Dat,
    /// Винительный
    Acc,
    /// Творительный
    Inst,
    /// Предложный
    Prep,
}

pub type Cases<T> = [T; 6];
