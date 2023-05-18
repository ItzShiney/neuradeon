use lazy_static::lazy_static;

pub fn amplifiers() -> &'static [&'static str] {
    lazy_static! {
        static ref AMPLIFIERS: Vec<&'static str> =
            vec![
                "адски",
                "максимально",
                // "пиздейше",
                "уёбищно",
            ];
    }

    &AMPLIFIERS
}
