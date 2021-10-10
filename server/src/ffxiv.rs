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

#[derive(Debug, Copy, Clone)]
pub enum Language {
    English,
    Japanese,
    German,
    French,
}

impl Language {
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
