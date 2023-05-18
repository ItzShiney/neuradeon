use super::Adjective;
use lazy_static::lazy_static;

macro_rules! ый {
    ($root:literal) => {
        [
            [
                concat!($root, "ый"),
                concat!($root, "ого"),
                concat!($root, "ому"),
                concat!($root, "ого"),
                concat!($root, "ым"),
                concat!($root, "ом"),
            ],
            [
                concat!($root, "ая"),
                concat!($root, "ой"),
                concat!($root, "ой"),
                concat!($root, "ую"),
                concat!($root, "ой"),
                concat!($root, "ой"),
            ],
            [
                concat!($root, "ое"),
                concat!($root, "ого"),
                concat!($root, "ому"),
                concat!($root, "ое"),
                concat!($root, "ым"),
                concat!($root, "ом"),
            ],
            [
                concat!($root, "ые"),
                concat!($root, "ых"),
                concat!($root, "ым"),
                concat!($root, "ые"),
                concat!($root, "ыми"),
                concat!($root, "ых"),
            ],
        ]
        .into()
    };
}

macro_rules! ый_postfixed {
    ($root:literal, $postfixed:literal) => {
        [
            [
                concat!($root, "ый ", $postfixed),
                concat!($root, "ого ", $postfixed),
                concat!($root, "ому ", $postfixed),
                concat!($root, "ого ", $postfixed),
                concat!($root, "ым ", $postfixed),
                concat!($root, "ом ", $postfixed),
            ],
            [
                concat!($root, "ая ", $postfixed),
                concat!($root, "ой ", $postfixed),
                concat!($root, "ой ", $postfixed),
                concat!($root, "ую ", $postfixed),
                concat!($root, "ой ", $postfixed),
                concat!($root, "ой ", $postfixed),
            ],
            [
                concat!($root, "ое ", $postfixed),
                concat!($root, "ого ", $postfixed),
                concat!($root, "ому ", $postfixed),
                concat!($root, "ое ", $postfixed),
                concat!($root, "ым ", $postfixed),
                concat!($root, "ом ", $postfixed),
            ],
            [
                concat!($root, "ые ", $postfixed),
                concat!($root, "ых ", $postfixed),
                concat!($root, "ым ", $postfixed),
                concat!($root, "ые ", $postfixed),
                concat!($root, "ыми ", $postfixed),
                concat!($root, "ых ", $postfixed),
            ],
        ]
        .into()
    };
}

macro_rules! concat_fix {
    ($root:literal, $ending:literal) => {
        const_str::replace!(const_str::replace!(concat!($root, $ending), "що", "ще"), "чо", "че")
    };
}

macro_rules! ий {
    ($root:literal) => {
        [
            [
                concat_fix!($root, "ий"),
                concat_fix!($root, "ого"),
                concat_fix!($root, "ому"),
                concat_fix!($root, "ого"),
                concat_fix!($root, "им"),
                concat_fix!($root, "ом"),
            ],
            [
                concat_fix!($root, "ая"),
                concat_fix!($root, "ой"),
                concat_fix!($root, "ой"),
                concat_fix!($root, "ую"),
                concat_fix!($root, "ой"),
                concat_fix!($root, "ой"),
            ],
            [
                concat_fix!($root, "ое"),
                concat_fix!($root, "ого"),
                concat_fix!($root, "ому"),
                concat_fix!($root, "ое"),
                concat_fix!($root, "им"),
                concat_fix!($root, "ом"),
            ],
            [
                concat_fix!($root, "ие"),
                concat_fix!($root, "их"),
                concat_fix!($root, "им"),
                concat_fix!($root, "их"),
                concat_fix!($root, "ими"),
                concat_fix!($root, "их"),
            ],
        ]
        .into()
    };
}

pub fn adjectives() -> &'static [Adjective] {
    lazy_static! {
        static ref ADJECTIVES: Vec<Adjective> = vec![
            ый!("обоссанн"),
            ый!("еблив"),
            ый!("пиздлив"),
            ый!("пизданут"),
            ый!("припизднут"),
            ый!("обдроченн"),
            ый!("обдолбанн"),
            ый!("объёбанн"),
            ый!("коксов"),
            ый!("передознут"),
            ый!("ёбан"),
            ый!("мёртв"),
            ый!("понадусёров"),
            ый!("переможн"),
            ый!("полноприводн"),
            ый!("заднеприводн"),
            ый!("грязн"),
            ый!("уёбищн"),
            ый!("говнососн"),
            ый!("дворов"),
            ый!("передавленн"),
            ый!("замкадышн"),
            ый!("подмосковн"),
            ый!("подсосн"),
            ый!("пиздоглаз"),
            ый!("подливн"),
            ый!("прозападн"),
            //
            ий!("пропутинск"),
            ий!("проукраинск"),
            ий!("укропск"),
            ий!("хохлятск"),
            ий!("прохохлятск"),
            ий!("прозеленск"),
            ий!("пропендосск"),
            ий!("прожидовск"),
            ий!("проблядск"),
            ий!("еврейск"),
            ий!("хуекрутящ"),
            ий!("минск"),
            ий!("вонюч"),
            // ий!("собач"),
            //
            ый_postfixed!("угашенн", "об дерево"),
        ];
    }

    &ADJECTIVES
}
