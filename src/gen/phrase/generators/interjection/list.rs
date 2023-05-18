use lazy_static::lazy_static;

pub fn interjections() -> &'static [&'static str] {
    lazy_static! {
        static ref INTERJECTIONS: Vec<&'static str> = vec!["блять", "нахуй", "сука"];
    }

    &INTERJECTIONS
}
