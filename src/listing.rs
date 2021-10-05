use std::borrow::Cow;
use bitflags::bitflags;
use ffxiv_types::jobs::{ClassJob, Class, Job};
use ffxiv_types::{Role, World};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sestring::SeString;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PartyFinderListing {
    pub id: u32,
    pub content_id_lower: u32,
    #[serde(with = "crate::base64_sestring")]
    pub name: SeString,
    #[serde(with = "crate::base64_sestring")]
    pub description: SeString,
    pub created_world: u8,
    pub home_world: u8,
    pub current_world: u8,
    pub category: DutyCategory,
    pub duty: u16,
    pub duty_type: DutyType,
    pub beginners_welcome: bool,
    pub seconds_remaining: u16,
    pub min_item_level: u16,
    pub num_parties: u8,
    pub slots_available: u8,
    pub objective: ObjectiveFlags,
    pub conditions: ConditionFlags,
    pub duty_finder_settings: DutyFinderSettingsFlags,
    pub loot_rules: LootRuleFlags,
    pub search_area: SearchAreaFlags,
    pub slots: Vec<PartyFinderSlot>,
    pub jobs_present: Vec<u8>,
}

impl PartyFinderListing {
    pub fn slots_filled(&self) -> usize {
        self.jobs_present.iter().filter(|&&job| job > 0).count()
    }

    pub fn is_cross_world(&self) -> bool {
        self.search_area.contains(SearchAreaFlags::DATA_CENTRE)
    }

    pub fn duty_name(&self) -> Cow<str> {
        match (&self.duty_type, &self.category) {
            (DutyType::Other, DutyCategory::Fates) => {
                if let Some(&name) = crate::ffxiv::TERRITORY_NAMES.get(&u32::from(self.duty)) {
                    return Cow::from(name);
                }

                return Cow::from("Fates");
            }
            (DutyType::Other, DutyCategory::TheHunt) => return Cow::from("The Hunt"),
            (DutyType::Other, DutyCategory::Duty) if self.duty == 0 => return Cow::from("None"),
            (DutyType::Normal, _) => {
                if let Some(info) = crate::ffxiv::DUTIES.get(&u32::from(self.duty)) {
                    return Cow::from(info.name);
                }
            }
            (DutyType::Roulette, _) => {
                if let Some(&name) = crate::ffxiv::ROULETTES.get(&u32::from(self.duty)) {
                    return Cow::from(name);
                }
            }
            _ => {}
        }

        Cow::from(format!("{:?}", self.category))
    }

    pub fn slots(&self) -> Vec<std::result::Result<ClassJob, (String, String)>> {
        let mut slots = Vec::with_capacity(self.slots_available as usize);
        for i in 0..self.slots_available as usize {
            if i >= self.jobs_present.len() {
                break;
            }

            let cj = match crate::ffxiv::JOBS.get(&u32::from(self.jobs_present[i])).copied() {
                Some(cj) => Ok(cj),
                None => Err((
                    self.slots[i].html_classes(),
                    self.slots[i].codes(),
                )),
            };
            slots.push(cj);
        }

        slots
    }

    pub fn created_world(&self) -> Option<World> {
        crate::ffxiv::WORLDS.get(&u32::from(self.created_world)).copied()
    }

    pub fn created_world_string(&self) -> Cow<str> {
        self.created_world()
            .map(|world| Cow::from(world.name()))
            .unwrap_or_else(|| Cow::from(self.created_world.to_string()))
    }

    pub fn home_world(&self) -> Option<World> {
        crate::ffxiv::WORLDS.get(&u32::from(self.home_world)).copied()
    }

    pub fn home_world_string(&self) -> Cow<str> {
        self.home_world()
            .map(|world| Cow::from(world.name()))
            .unwrap_or_else(|| Cow::from(self.home_world.to_string()))
    }

    pub fn prepend_flags(&self) -> (&'static str, String) {
        let mut colour_class = "";
        let mut flags = Vec::new();

        if self.objective.contains(ObjectiveFlags::PRACTICE) {
            flags.push("[Practice]");
            colour_class = "desc-green";
        }

        if self.objective.contains(ObjectiveFlags::DUTY_COMPLETION) {
            flags.push("[Duty Completion]");
            colour_class = "desc-blue";
        }

        if self.objective.contains(ObjectiveFlags::LOOT) {
            flags.push("[Loot]");
            colour_class = "desc-yellow";
        }

        if self.conditions.contains(ConditionFlags::DUTY_COMPLETE) {
            flags.push("[Duty Complete]");
        }

        if self.conditions.contains(ConditionFlags::DUTY_INCOMPLETE) {
            flags.push("[Duty Incomplete]");
        }

        if self.search_area.contains(SearchAreaFlags::ONE_PLAYER_PER_JOB) {
            flags.push("[One Player per Job]");
        }

        (colour_class, flags.join(""))
    }

    pub fn data_centre_name(&self) -> Option<&'static str> {
        crate::ffxiv::WORLDS.get(&u32::from(self.created_world))
            .map(|w| w.data_center().name())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PartyFinderSlot {
    pub accepting: JobFlags,
}

impl PartyFinderSlot {
    pub fn html_classes(&self) -> String {
        if self.accepting == JobFlags::all() {
            return "empty".into();
        }

        let mut classes = Vec::with_capacity(3);
        let cjs = self.accepting.classjobs();

        if cjs.iter().any(|cj| cj.role() == Some(Role::Healer)) {
            classes.push("healer");
        }

        if cjs.iter().any(|cj| cj.role() == Some(Role::Tank)) {
            classes.push("tank");
        }

        if cjs.iter().any(|cj| cj.role() == Some(Role::Dps)) {
            classes.push("dps");
        }

        classes.join(" ")
    }

    pub fn codes(&self) -> String {
        self.accepting.classjobs()
            .iter()
            .map(|cj| cj.code())
            .intersperse(" ")
            .collect()
    }
}

#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u32)]
pub enum DutyCategory {
    Duty = 0,
    QuestBattles = 1 << 0,
    Fates = 1 << 1,
    TreasureHunt = 1 << 2,
    TheHunt = 1 << 3,
    GatheringForays = 1 << 4,
    DeepDungeons = 1 << 5,
    AdventuringForays = 1 << 6,
}

impl DutyCategory {
    pub fn as_u32(self) -> u32 {
        unsafe { std::mem::transmute(self) }
    }
}

#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum DutyType {
    Other = 0,
    Roulette = 1 << 0,
    Normal = 1 << 1,
}

impl DutyType {
    pub fn as_u8(self) -> u8 {
        unsafe { std::mem::transmute(self) }
    }
}

bitflags! {
    #[derive(Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct ObjectiveFlags : u32 {
        const NONE = 0;
        const DUTY_COMPLETION = 1 << 0;
        const PRACTICE = 1 << 1;
        const LOOT = 1 << 2;
    }
}

bitflags! {
    #[derive(Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct ConditionFlags : u32 {
        const NONE = 1 << 0;
        const DUTY_COMPLETE = 1 << 1;
        const DUTY_INCOMPLETE = 1 << 2;
    }
}

bitflags! {
    #[derive(Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct DutyFinderSettingsFlags : u32 {
        const NONE = 0;
        const UNDERSIZED_PARTY = 1 << 0;
        const MINIMUM_ITEM_LEVEL = 1 << 1;
        const SILENCE_ECHO = 1 << 2;
    }
}

bitflags! {
    #[derive(Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct LootRuleFlags : u32 {
        const NONE = 0;
        const GREED_ONLY = 1 << 0;
        const LOOTMASTER = 1 << 1;
    }
}

bitflags! {
    #[derive(Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct SearchAreaFlags : u32 {
        const DATA_CENTRE = 1 << 0;
        const PRIVATE = 1 << 1;
        const ALLIANCE_RAID = 1 << 2;
        const WORLD = 1 << 3;
        const ONE_PLAYER_PER_JOB = 1 << 5;
    }
}

bitflags! {
    #[derive(Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct JobFlags : u32 {
        const GLADIATOR = 1 << 1;
        const PUGILIST = 1 << 2;
        const MARAUDER = 1 << 3;
        const LANCER = 1 << 4;
        const ARCHER = 1 << 5;
        const CONJURER = 1 << 6;
        const THAUMATURGE = 1 << 7;
        const PALADIN = 1 << 8;
        const MONK = 1 << 9;
        const WARRIOR = 1 << 10;
        const DRAGOON = 1 << 11;
        const BARD = 1 << 12;
        const WHITE_MAGE = 1 << 13;
        const BLACK_MAGE = 1 << 14;
        const ARCANIST = 1 << 15;
        const SUMMONER = 1 << 16;
        const SCHOLAR = 1 << 17;
        const ROGUE = 1 << 18;
        const NINJA = 1 << 19;
        const MACHINIST = 1 << 20;
        const DARK_KNIGHT = 1 << 21;
        const ASTROLOGIAN = 1 << 22;
        const SAMURAI = 1 << 23;
        const RED_MAGE = 1 << 24;
        const BLUE_MAGE = 1 << 25;
        const GUNBREAKER = 1 << 26;
        const DANCER = 1 << 27;
    }
}

impl JobFlags {
    pub fn classjobs(&self) -> Vec<ClassJob> {
        let mut cjs = Vec::new();

        if self.contains(Self::GLADIATOR) {
            cjs.push(ClassJob::Class(Class::Gladiator));
        }

        if self.contains(Self::PUGILIST) {
            cjs.push(ClassJob::Class(Class::Pugilist));
        }

        if self.contains(Self::MARAUDER) {
            cjs.push(ClassJob::Class(Class::Marauder));
        }

        if self.contains(Self::LANCER) {
            cjs.push(ClassJob::Class(Class::Lancer));
        }

        if self.contains(Self::ARCHER) {
            cjs.push(ClassJob::Class(Class::Archer));
        }

        if self.contains(Self::CONJURER) {
            cjs.push(ClassJob::Class(Class::Conjurer));
        }

        if self.contains(Self::THAUMATURGE) {
            cjs.push(ClassJob::Class(Class::Thaumaturge));
        }

        if self.contains(Self::PALADIN) {
            cjs.push(ClassJob::Job(Job::Paladin));
        }

        if self.contains(Self::MONK) {
            cjs.push(ClassJob::Job(Job::Monk));
        }

        if self.contains(Self::WARRIOR) {
            cjs.push(ClassJob::Job(Job::Warrior));
        }

        if self.contains(Self::DRAGOON) {
            cjs.push(ClassJob::Job(Job::Dragoon));
        }

        if self.contains(Self::BARD) {
            cjs.push(ClassJob::Job(Job::Bard));
        }

        if self.contains(Self::WHITE_MAGE) {
            cjs.push(ClassJob::Job(Job::WhiteMage));
        }

        if self.contains(Self::BLACK_MAGE) {
            cjs.push(ClassJob::Job(Job::BlackMage));
        }

        if self.contains(Self::ARCANIST) {
            cjs.push(ClassJob::Class(Class::Arcanist));
        }

        if self.contains(Self::SUMMONER) {
            cjs.push(ClassJob::Job(Job::Summoner));
        }

        if self.contains(Self::SCHOLAR) {
            cjs.push(ClassJob::Job(Job::Scholar));
        }

        if self.contains(Self::ROGUE) {
            cjs.push(ClassJob::Class(Class::Rogue));
        }

        if self.contains(Self::NINJA) {
            cjs.push(ClassJob::Job(Job::Ninja));
        }

        if self.contains(Self::MACHINIST) {
            cjs.push(ClassJob::Job(Job::Machinist));
        }

        if self.contains(Self::DARK_KNIGHT) {
            cjs.push(ClassJob::Job(Job::DarkKnight));
        }

        if self.contains(Self::ASTROLOGIAN) {
            cjs.push(ClassJob::Job(Job::Astrologian));
        }

        if self.contains(Self::SAMURAI) {
            cjs.push(ClassJob::Job(Job::Samurai));
        }

        if self.contains(Self::RED_MAGE) {
            cjs.push(ClassJob::Job(Job::RedMage));
        }

        if self.contains(Self::BLUE_MAGE) {
            cjs.push(ClassJob::Job(Job::BlueMage));
        }

        if self.contains(Self::GUNBREAKER) {
            cjs.push(ClassJob::Job(Job::Gunbreaker));
        }

        if self.contains(Self::DANCER) {
            cjs.push(ClassJob::Job(Job::Dancer));
        }

        cjs
    }
}
