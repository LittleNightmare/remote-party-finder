use std::borrow::Cow;
use std::collections::HashMap;
use sestring::SeString;
use serde::{Deserialize, Deserializer};
use crate::ffxiv::Language;
use crate::listing::{DutyCategory, DutyType};

#[derive(Debug, Clone, Deserialize)]
pub struct Statistics {
    pub count: Vec<Count>,
    #[serde(deserialize_with = "alias_de")]
    pub aliases: HashMap<u32, Vec<Alias>>,
    pub duties: Vec<DutyInfo>,
    pub hosts: Vec<HostInfo>,
    pub hours: Vec<HourInfo>,
    pub days: Vec<DayInfo>,
}

fn alias_de<'de, D>(de: D) -> std::result::Result<HashMap<u32, Vec<Alias>>, D::Error>
    where D: Deserializer<'de>
{
    let aliases: Vec<AliasInfo> = Deserialize::deserialize(de)?;
    let map = aliases
        .into_iter()
        .map(|info| (info.content_id_lower, info.aliases))
        .collect();
    Ok(map)
}

impl Statistics {
    pub fn num_listings(&self) -> usize {
        self.count[0].count
    }

    pub fn player_name(&self, cid: &u32) -> Cow<str> {
        let aliases = match self.aliases.get(cid) {
            Some(a) => a,
            None => return "<unknown>".into(),
        };

        if aliases.is_empty() {
            return "<unknown>".into();
        }

        let world = match crate::ffxiv::WORLDS.get(&aliases[0].home_world) {
            Some(world) => world.name(),
            None => "<unknown>",
        };

        format!("{} @ {}", aliases[0].name.text(), world).into()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Count {
    pub count: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AliasInfo {
    #[serde(rename = "_id")]
    pub content_id_lower: u32,
    pub aliases: Vec<Alias>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Alias {
    #[serde(with = "crate::base64_sestring")]
    pub name: SeString,
    pub home_world: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DutyInfo {
    #[serde(rename = "_id")]
    pub info: (u8, u32, u16),
    pub count: usize,
}

impl DutyInfo {
    pub fn name(&self, lang: &Language) -> Cow<str> {
        let kind = match DutyType::from_u8(self.info.0) {
            Some(k) => k,
            None => return Cow::from("<unknown>"),
        };
        let category = match DutyCategory::from_u32(self.info.1) {
            Some(c) => c,
            None => return Cow::from("<unknown>"),
        };
        crate::ffxiv::duty_name(kind, category, self.info.2, *lang)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct HostInfo {
    #[serde(rename = "_id")]
    pub content_id_lower: u32,
    pub count: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HourInfo {
    #[serde(rename = "_id")]
    pub hour: u8,
    pub count: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DayInfo {
    #[serde(rename = "_id")]
    pub day: u8,
    pub count: usize,
}

impl DayInfo {
    pub fn name(&self) -> &'static str {
        match self.day {
            1 => "Sunday",
            2 => "Monday",
            3 => "Tuesday",
            4 => "Wednesday",
            5 => "Thursday",
            6 => "Friday",
            7 => "Saturday",
            _ => "<unknown>",
        }
    }
}
