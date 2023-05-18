mod interjection;

use crate::config;
use crate::gen::adjective;
use crate::gen::object;
use crate::Case;
use crate::Gender;
pub use interjection::*;
use lazy_static::lazy_static;

macro_rules! phrase_generator {
    (
        let $gender_ident:ident = ?;
        $( let $intj:ident = ?; )?
        [$expr1:expr $(, $exprs:expr)* $(,)?]
    ) => {
        || -> String {
            let $gender_ident = $crate::Gender::choose();
            $( let $intj = maybe_interjection(); )?

            let mut res = format!("{}", $expr1);
            $(
                res += format!(" {}", $exprs).as_str();
            )*

            res = res.replace("  ", " ").replace(" ,", ",").replace(" .", ".").replace(" ?", "?").into();
            res
        }
    };
}

macro_rules! gendered {
    ($gender:ident, $m:expr, $f:expr, $n:expr, $p:expr) => {
        match $gender {
            $crate::Gender::Masculine => $m,
            $crate::Gender::Feminine => $f,
            $crate::Gender::Neuter => $n,
            $crate::Gender::Plural => $p,
        }
    };
}

macro_rules! numbered {
    ($gender:ident, $sg:expr, $pl:expr) => {
        match $gender.number() {
            $crate::Number::Singular => $sg,
            $crate::Number::Plural => $pl,
        }
    };
}

pub fn phrase_generators() -> &'static [fn() -> String] {
    lazy_static! {
        static ref PHRASE_GENERATORS: Vec<fn() -> String> = {
            let mut res: Vec<fn() -> String> = vec![
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "ты", "вы"],
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "ты", "вы"],
                        interjection(),
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        "ненавижу",
                        numbered![gender, "тебя", "вас"],
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        "ненавижу",
                        numbered![gender, "тебя", "вас"],
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "пошёл нахуй", "пошли нахуй"],
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "ливни из жизни,", "ливните из жизни,"],
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "ты еблан?", "вы ебланы?"],
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "ты вылупился", "вы вылупились"],
                        "из яйца мёртвого страуса,",
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        gendered![gender, "какой же ты", "какая же ты", "какое же ты", "какие же вы"],
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "лети", "летите"],
                        "в чс",
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "заткни свой рот", "заткните свои рты"],
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "заткнул своё грязное ебало,", "заткнули свои грязные ёбла,"],
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        "нихуя не смешно,",
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        "абсолютно не смешно,",
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        "удачной поездки в бахмут,",
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        "удачи сдохнуть под бахмутом,",
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        "ну теперь",
                        numbered![gender, "ты", "вы"],
                        "для меня не лучше",
                        object(gender, Case::Gen),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "заебал", "заебали"],
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        "как же",
                        numbered![gender, "ты заебал", "вы заебали"],
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "заебал", "заебали"],
                        "с этой припизднутой хуйнёй,",
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "поплачь", "поплачьте"],
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "вся твоя семья гнила", "ваши семьи гнили"],
                        "в ёбаной братской могиле под донецком ещё в 2014,",
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "твой отец кололся", "ваши отцы кололись"],
                        "некачественным донецким героином и",
                        numbered![gender, "сдох на твоих", "сдохли на ваших"],
                        // "вонючих",
                        adjective(Gender::Plural, Case::Gen),
                        "руках,",
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "твой отец кололся", "ваши отцы кололись"],
                        "некачественным донецким героином и",
                        numbered![gender, "сдох на твоих", "сдохли на ваших"],
                        // "вонючих",
                        adjective(Gender::Plural, Case::Gen),
                        "руках",
                        intj.unwrap_or_default(),
                        ",",
                        object(gender, Case::Nom),
                        ", после чего",
                        numbered![gender, "ты пошёл по его", "вы пошли по их"],
                        "стопам и",
                        numbered![gender, "вступил", "вступили"],
                        "в народную милицию лнр, где и",
                        numbered![gender, "стал", "стали"],
                        adjective(numbered![gender, Gender::Masculine, Gender::Plural], Case::Inst),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "думаешь самый умный", "думаете самые умные"],
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                        "?"
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "может тебе ебало завалить,", "может вам ебальники завалить,"],
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                        "?"
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "ты блять сдохнешь когда-то?", "вы блять сдохнете когда-то?"],
                        object(gender, Case::Nom),
                        intj.unwrap_or_default(),
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        object(gender, Case::Nom),
                        numbered![gender, "у тебя за щекой", "у вас за щеками"],
                        intj.unwrap_or_default(),
                        ",",
                        numbered![gender, "проверяй", "проверяйте"],
                        ". нет ничего? ясно, сглотнул уже",
                    ]
                },
                phrase_generator! {
                    let gender = ?;
                    let intj = ?;
                    [
                        numbered![gender, "твой отец был", "ваши отцы были"],
                        object(gender, Case::Inst),
                        ",",
                        numbered![gender, "он метал", "они метали"],
                        "куски второсортного говница в",
                        numbered![gender, "твой рот", "ваши рты"],
                        ", поэтому",
                        numbered![gender, "тебя", "вас"],
                        "и прозвали в школе",
                        object(gender, Case::Nom),
                        // numbered![gender, "приёмник", "приёмники"],
                        // "второсортного дерьма",
                        "и трахали в рот",
                        intj.unwrap_or_default(),
                    ]
                },
            ];

            if config().completely_out_of_context_phrases {
                res.extend([
                    phrase_generator! {
                        let gender = ?;
                        let intj = ?;
                        [
                            "нахуй",
                            gendered![gender, "проект этого", "проект этой", "проект этого", "проекты этих"],
                            object(gender, Case::Gen),
                            "на гитхабе,",
                            object(gender, Case::Nom),
                            intj.unwrap_or_default(),
                            "?",
                        ]
                    },
                    phrase_generator! {
                        let gender = ?;
                        let intj = ?;
                        [
                            "нахуй",
                            numbered![gender, "твой проект", "ваши проекты"],
                            "на гитхабе,",
                            object(gender, Case::Nom),
                            intj.unwrap_or_default(),
                            "?",
                        ]
                    },
                ]);
            }

            if config().self_deprecation {
                res.extend([
                    phrase_generator! {
                        let gender = ?;
                        [
                            numbered![gender, "я", "мы"],
                            object(gender, Case::Nom),
                        ]
                    },
                    phrase_generator! {
                        let gender = ?;
                        [
                            gendered![gender, "какой же я", "какая же я", "какое же я", "какие же мы"],
                            object(gender, Case::Nom),
                        ]
                    },
                ]);
            }

            res
        };
    }

    &PHRASE_GENERATORS
}
