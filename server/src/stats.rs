use std::borrow::Cow;
use std::collections::HashMap;
use sestring::SeString;
use serde::{Deserialize, Deserializer};
use crate::ffxiv::Language;
use crate::listing::{DutyCategory, DutyType};

#[derive(Debug, Clone, Deserialize)]
pub struct CachedStatistics {
    pub all_time: Statistics,
    pub seven_days: Statistics,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Aliases {
    #[serde(deserialize_with = "alias_de")]
    pub aliases: HashMap<u32, Alias>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Statistics {
    pub count: Vec<Count>,
    #[serde(default)]
    pub aliases: HashMap<u32, Alias>,
    pub duties: Vec<DutyInfo>,
    pub hosts: Vec<HostInfo>,
    pub hours: Vec<HourInfo>,
    pub days: Vec<DayInfo>,
}

fn alias_de<'de, D>(de: D) -> std::result::Result<HashMap<u32, Alias>, D::Error>
where D: Deserializer<'de>
{
    let aliases: Vec<AliasInfo> = Deserialize::deserialize(de)?;
    let map = aliases
        .into_iter()
        .map(|info| (info.content_id, info.alias))
        .collect();
    Ok(map)
}

impl Statistics {
    pub fn num_listings(&self) -> usize {
        if self.count.is_empty() {
            return 0;
        }

        self.count[0].count
    }

    pub fn player_name(&self, cid: &u32) -> Cow<str> {
        let alias = match self.aliases.get(cid) {
            Some(a) => a,
            None => return "<unknown>".into(),
        };

        let world = match crate::ffxiv::WORLDS.get(&alias.home_world) {
            Some(world) => world.name(),
            None => "<unknown>",
        };

        format!("{} @ {}", alias.name.text(), world).into()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Count {
    pub count: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AliasInfo {
    #[serde(rename = "_id")]
    pub content_id: u32,
    pub alias: Alias,
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
    pub created_world: u32,
    pub count: usize,
    pub content_ids: Vec<HostInfoInfo>,
}

impl HostInfo {
    pub fn num_other(&self) -> usize {
        let top15: usize = self.content_ids.iter().map(|info| info.count).sum();
        self.count - top15
    }

    pub fn world_name(&self) -> &'static str {
        match crate::ffxiv::WORLDS.get(&self.created_world) {
            Some(world) => world.name(),
            None => "<unknown>",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct HostInfoInfo {
    pub content_id: u32,
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
