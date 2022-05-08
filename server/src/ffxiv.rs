pub mod auto_translate;
pub mod duties;
pub mod jobs;
pub mod roulettes;
pub mod territory_names;
pub mod treasure_maps;
pub mod worlds;

pub use self::{
    auto_translate::AUTO_TRANSLATE,
    duties::DUTIES,
    jobs::JOBS,
    roulettes::ROULETTES,
    territory_names::TERRITORY_NAMES,
    treasure_maps::TREASURE_MAPS,
    worlds::WORLDS,
};

use std::{
    cmp::Ordering,
    str::FromStr,
};
use std::borrow::Cow;
use crate::listing::{DutyCategory, DutyType};

#[derive(Debug, Copy, Clone)]
pub enum Language {
    English,
    Japanese,
    German,
    French,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Japanese => "ja",
            Self::German => "de",
            Self::French => "fr",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::English => "english",
            Self::Japanese => "日本語",
            Self::German => "deutsch",
            Self::French => "français",
        }
    }

    pub fn from_codes(val: Option<&str>) -> Self {
        let val = match val {
            Some(v) => v,
            None => return Self::English,
        };

        let mut parts: Vec<(&str, f32)> = val.split(',')
            .map(|part| {
                let sub_parts: Vec<&str> = part.split(';').collect();
                if sub_parts.len() == 1 {
                    (sub_parts[0], 1.0)
                } else if let Ok(val) = f32::from_str(sub_parts[0]) {
                    (sub_parts[0], val)
                } else {
                    (sub_parts[0], 0.0)
                }
            })
            .collect();
        parts.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Less));
        if parts.len() == 0 {
            return Self::English;
        }

        for (lang, _) in parts {
            let first = lang.split('-').next().unwrap();
            match first {
                "en" => return Self::English,
                "ja" => return Self::Japanese,
                "de" => return Self::German,
                "fr" => return Self::French,
                _ => {},
            }
        }

        Self::English
    }
}

#[derive(Debug)]
pub struct LocalisedText {
    pub en: &'static str,
    pub ja: &'static str,
    pub de: &'static str,
    pub fr: &'static str,
}

impl LocalisedText {
    pub fn text(&self, lang: &Language) -> &'static str {
        match lang {
            Language::English => self.en,
            Language::Japanese => self.ja,
            Language::German => self.de,
            Language::French => self.fr,
        }
    }
}

pub fn duty_name<'a>(duty_type: DutyType, category: DutyCategory, duty: u16, lang: Language) -> Cow<'a, str> {
    match (duty_type, category) {
        (DutyType::Other, DutyCategory::Fates) => {
            if let Some(name) = crate::ffxiv::TERRITORY_NAMES.get(&u32::from(duty)) {
                return Cow::from(name.text(&lang));
            }

            return Cow::from("FATEs");
        }
        (DutyType::Other, DutyCategory::TheHunt) => return Cow::from(match lang {
            Language::English => "The Hunt",
            Language::Japanese => "モブハント",
            Language::German => "Hohe Jagd",
            Language::French => "Contrats de chasse",
        }),
        (DutyType::Other, DutyCategory::Duty) if duty == 0 => return Cow::from(match lang {
            Language::English => "None",
            Language::Japanese => "設定なし",
            Language::German => "Nicht festgelegt",
            Language::French => "Non spécifiée",
        }),
        (DutyType::Other, DutyCategory::DeepDungeons) if duty == 1 => return Cow::from(match lang {
            Language::English => "The Palace of the Dead",
            Language::Japanese => "死者の宮殿",
            Language::German => "Palast der Toten",
            Language::French => "Palais des morts",
        }),
        (DutyType::Other, DutyCategory::DeepDungeons) if duty == 2 => return Cow::from(match lang {
            Language::English => "Heaven-on-High",
            Language::Japanese => "アメノミハシラ",
            Language::German => "Himmelssäule",
            Language::French => "Pilier des Cieux",
        }),
        (DutyType::Normal, _) => {
            if let Some(info) = crate::ffxiv::DUTIES.get(&u32::from(duty)) {
                return Cow::from(info.name.text(&lang));
            }
        }
        (DutyType::Roulette, _) => {
            if let Some(info) = crate::ffxiv::ROULETTES.get(&u32::from(duty)) {
                return Cow::from(info.name.text(&lang));
            }
        }
        (_, DutyCategory::QuestBattles) => return Cow::from(match lang {
            Language::English => "Quest Battles",
            Language::Japanese => "クエストバトル",
            Language::German => "Auftragskampf",
            Language::French => "Batailles de quête",
        }),
        (_, DutyCategory::TreasureHunt) => if let Some(name) = crate::ffxiv::TREASURE_MAPS.get(&u32::from(duty)) {
            return Cow::from(name.text(&lang));
        }
        _ => {}
    }

    Cow::from(format!("{:?}", category))
}
