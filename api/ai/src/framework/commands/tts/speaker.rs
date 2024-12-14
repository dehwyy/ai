use std::fmt::Display;

use serde::{Serialize, Serializer};

#[allow(dead_code)]
const RU_SPEAKERS: [&str; 5] = ["aidar", "baya", "kseniya", "xenia", "eugene"];
// const EN_SPEAKERS: en_0, en_1, ..., en_117

const RU_MAN: &str = "aidar";
const RU_WOMEN: &str = "kseniya";

const EN_MAN: &str = "en_70";
const EN_WOMEN: &str = "en_0";

pub enum Speaker {
    RuMan,
    RuWoman,
    EnMan,
    EnWoman,
}

impl Serialize for Speaker {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Speaker::RuMan => s.serialize_str(RU_MAN),
            Speaker::RuWoman => s.serialize_str(RU_WOMEN),
            Speaker::EnMan => s.serialize_str(EN_MAN),
            Speaker::EnWoman => s.serialize_str(EN_WOMEN),
        }
    }
}

// impl TryFrom<String> for Speaker {
//     type Error = String;

//     fn try_from(mut value: String) -> Result<Self, Self::Error> {
//         if RU_SPEAKERS.contains(&value.as_str()) {
//             return match &value {
//                 v if v == RU_MAN => Ok(Speaker::RuMan),
//                 v if v == RU_WOMEN => Ok(Speaker::RuWoman),
//                 _ => Err(format!("Unknown speaker: {}", value)),
//             };

//             // return Ok(Speaker(value));
//         }

//         // Try to parse as ENG speaker.
//         if value.starts_with("en_") {
//             match &value {
//                 v if v == EN_MAN => value = EN_MAN_DEFAULT.into(),
//                 v if v == EN_WOMEN => value = EN_WOMEN_DEFAULT.into(),
//                 _ => (),
//             }

//             let speaker_uid = value[3..].parse::<u8>().map_err(|e| format!("{e:?}"))?;

//             if (0..=117).contains(&speaker_uid) {
//                 return Ok(Speaker(value));
//             }
//         }

//         Err(format!("Unknown speaker: {}", value))
//     }
// }

impl Display for Speaker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Speaker::RuMan => RU_MAN,
            Speaker::RuWoman => RU_WOMEN,
            Speaker::EnMan => EN_MAN,
            Speaker::EnWoman => EN_WOMEN,
        };
        write!(f, "{}", s)
    }
}
