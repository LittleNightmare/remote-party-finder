use std::borrow::Cow;
use bitflags::bitflags;
use ffxiv_types::jobs::{ClassJob, Class, Job};
use ffxiv_types::{Role, World};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sestring::SeString;
use crate::ffxiv::duties::{ContentKind, DutyInfo};
use crate::ffxiv::{Language, LocalisedText};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PartyFinderListing {
    pub id: u32,
    pub content_id_lower: u32,
    #[serde(with = "crate::base64_sestring")]
    pub name: SeString,
    #[serde(with = "crate::base64_sestring")]
    pub description: SeString,
    pub created_world: u16,
    pub home_world: u16,
    pub current_world: u16,
    pub category: DutyCategory,
    pub duty: u16,
    pub duty_type: DutyType,
    pub beginners_welcome: bool,
    pub seconds_remaining: u16,
    pub min_item_level: u16,
    pub num_parties: u8,
    pub slots_available: u8,
    pub last_server_restart: u32,
    pub objective: ObjectiveFlags,
    pub conditions: ConditionFlags,
    pub duty_finder_settings: DutyFinderSettingsFlags,
    pub loot_rules: LootRuleFlags,
    pub search_area: SearchAreaFlags,
    pub slots: Vec<PartyFinderSlot>,
    pub jobs_present: Vec<u8>,
}

#[allow(unused)]
impl PartyFinderListing {
    pub fn slots_filled(&self) -> usize {
        self.jobs_present.iter().filter(|&&job| job > 0).count()
    }

    pub fn is_cross_world(&self) -> bool {
        self.search_area.contains(SearchAreaFlags::DATA_CENTRE)
    }

    pub fn duty_name(&self, lang: &Language) -> Cow<str> {
        crate::ffxiv::duty_name(self.duty_type, self.category, self.duty, *lang)
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

        if self.conditions.contains(ConditionFlags::DUTY_COMPLETE_WEEKLY_REWARD_UNCLAIMED) {
            flags.push("[Duty Complete (Weekly Reward Unclaimed)]")
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

    pub fn high_end(&self) -> bool {
        if self.duty_type != DutyType::Normal {
            return false;
        }

        crate::ffxiv::duty(u32::from(self.duty))
            .map(|info| info.high_end)
            .unwrap_or_default()
    }

    pub fn content_kind(&self) -> u32 {
        if self.duty_type != DutyType::Normal {
            return 0;
        }

        crate::ffxiv::duty(u32::from(self.duty))
            .map(|info| info.content_kind.as_u32())
            .unwrap_or_default()
    }

    pub fn pf_category(&self) -> PartyFinderCategory {
        self.category.pf_category()
    }

    pub fn html_pf_category(&self) -> &'static str {
        self.pf_category().as_str()
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
    None = 0 << 0,
    DutyRoulette = 1 << 1,
    Dungeon = 1 << 2,
    Guildhest = 1 << 3,
    Trial = 1 << 4,
    Raid = 1 << 5,
    HighEndDuty = 1 << 6,
    PvP = 1 << 7,
    GoldSaucer = 1 << 8,
    Fate = 1 << 9,
    TreasureHunt = 1 << 10,
    TheHunt = 1 << 11,
    GatheringForay = 1 << 12,
    DeepDungeon = 1 << 13,
    FieldOperation = 1 << 14,
    VariantAndCriterionDungeon = 1 << 15,
}

#[allow(unused)]
impl DutyCategory {
    pub fn from_u32(u: u32) -> Option<Self> {
        Some(match u {
            0 => Self::None,
            1 => Self::DutyRoulette,
            2 => Self::Dungeon,
            4 => Self::Guildhest,
            8 => Self::Trial,
            16 => Self::Raid,
            32 => Self::HighEndDuty,
            64 => Self::PvP,
            128 => Self::GoldSaucer,
            256 => Self::Fate,
            512 => Self::TreasureHunt,
            1024 => Self::TheHunt,
            2048 => Self::GatheringForay,
            4096 => Self::DeepDungeon,
            8192 => Self::FieldOperation,
            16384 => Self::VariantAndCriterionDungeon,
            _ => return None,
        })
    }

    pub fn pf_category(&self) -> PartyFinderCategory {
        match self {
            DutyCategory::None => PartyFinderCategory::None,
            DutyCategory::DutyRoulette => PartyFinderCategory::DutyRoulette,
            DutyCategory::Dungeon => PartyFinderCategory::Dungeons,
            DutyCategory::Guildhest => PartyFinderCategory::Guildhests,
            DutyCategory::Trial => PartyFinderCategory::Trials,
            DutyCategory::Raid => PartyFinderCategory::Raids,
            DutyCategory::HighEndDuty => PartyFinderCategory::HighEndDuty,
            DutyCategory::PvP => PartyFinderCategory::Pvp,
            DutyCategory::GoldSaucer => PartyFinderCategory::GoldSaucer,
            DutyCategory::Fate => PartyFinderCategory::Fates,
            DutyCategory::TreasureHunt => PartyFinderCategory::TreasureHunt,
            DutyCategory::TheHunt => PartyFinderCategory::TheHunt,
            DutyCategory::GatheringForay => PartyFinderCategory::GatheringForays,
            DutyCategory::DeepDungeon => PartyFinderCategory::DeepDungeons,
            DutyCategory::FieldOperation => PartyFinderCategory::FieldOperations,
            DutyCategory::VariantAndCriterionDungeon => PartyFinderCategory::VariantAndCriterionDungeonFinder,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum DutyType {
    Other = 0,
    Roulette = 1 << 0,
    Normal = 1 << 1,
}

#[allow(unused)]
impl DutyType {
    pub fn as_u8(self) -> u8 {
        unsafe { std::mem::transmute(self) }
    }

    pub fn from_u8(u: u8) -> Option<Self> {
        Some(match u {
            0 => Self::Other,
            1 => Self::Roulette,
            2 => Self::Normal,
            _ => return None,
        })
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
        const DUTY_COMPLETE_WEEKLY_REWARD_UNCLAIMED = 1 << 3;
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
        const REAPER = 1 << 28;
        const SAGE = 1 << 29;
        const VIPER = 1 << 30;
        const PICTOMANCER = 1 << 31;
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

        if self.contains(Self::REAPER) {
            cjs.push(ClassJob::Job(Job::Reaper));
        }

        if self.contains(Self::SAGE) {
            cjs.push(ClassJob::Job(Job::Sage));
        }

        if self.contains(Self::VIPER) {
            cjs.push(ClassJob::Job(Job::Viper));
        }

        if self.contains(Self::PICTOMANCER) {
            cjs.push(ClassJob::Job(Job::Pictomancer));
        }

        cjs
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum PartyFinderCategory {
    DutyRoulette,
    Dungeons,
    Guildhests,
    Trials,
    Raids,
    HighEndDuty,
    Pvp,
    GoldSaucer,
    Fates,
    TreasureHunt,
    TheHunt,
    GatheringForays,
    DeepDungeons,
    FieldOperations,
    VariantAndCriterionDungeonFinder,
    None,
}

impl PartyFinderCategory {
    pub const ALL: [Self; 16] = [
        Self::DutyRoulette,
        Self::Dungeons,
        Self::Guildhests,
        Self::Trials,
        Self::Raids,
        Self::HighEndDuty,
        Self::Pvp,
        Self::GoldSaucer,
        Self::Fates,
        Self::TreasureHunt,
        Self::TheHunt,
        Self::GatheringForays,
        Self::DeepDungeons,
        Self::FieldOperations,
        Self::VariantAndCriterionDungeonFinder,
        Self::None,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::DutyRoulette => "DutyRoulette",
            Self::Dungeons => "Dungeons",
            Self::Guildhests => "Guildhests",
            Self::Trials => "Trials",
            Self::Raids => "Raids",
            Self::HighEndDuty => "HighEndDuty",
            Self::Pvp => "Pvp",
            Self::GoldSaucer => "GoldSaucer",
            Self::Fates => "Fates",
            Self::TreasureHunt => "TreasureHunt",
            Self::TheHunt => "TheHunt",
            Self::GatheringForays => "GatheringForays",
            Self::DeepDungeons => "DeepDungeons",
            Self::FieldOperations => "AdventuringForays",
            Self::VariantAndCriterionDungeonFinder => "V&C Dungeon Finder",
            Self::None => "None",
        }
    }

    pub fn name(self) -> LocalisedText {
        match self {
            Self::DutyRoulette => LocalisedText {
                en: "Duty Roulette",
                ja: "コンテンツルーレット",
                de: "Zufallsinhalte",
                fr: "Missions aléatoires",
            },
            Self::Dungeons => LocalisedText {
                en: "Dungeons",
                ja: "ダンジョン",
                de: "Dungeons",
                fr: "Donjons",
            },
            Self::Guildhests => LocalisedText {
                en: "Guildhests",
                ja: "ギルドオーダー",
                de: "Gildengeheiße",
                fr: "Opérations de guilde",
            },
            Self::Trials => LocalisedText {
                en: "Trials",
                ja: "討伐・討滅戦",
                de: "Prüfungen",
                fr: "Défis",
            },
            Self::Raids => LocalisedText {
                en: "Raids",
                ja: "レイド",
                de: "Raids",
                fr: "Raids",
            },
            Self::HighEndDuty => LocalisedText {
                en: "High-end Duty",
                ja: "高難易度コンテンツ",
                de: "Schwierige Inhalte",
                fr: "Missions à difficulté élevée",
            },
            Self::Pvp => LocalisedText {
                en: "PvP",
                ja: "PvP",
                de: "PvP",
                fr: "JcJ",
            },
            Self::GoldSaucer => LocalisedText {
                en: "Gold Saucer",
                ja: "ゴールドソーサー",
                de: "Gold Saucer",
                fr: "Gold Saucer",
            },
            Self::Fates => LocalisedText {
                en: "FATEs",
                ja: "F.A.T.E.",
                de: "FATEs",
                fr: "ALÉA",
            },
            Self::TreasureHunt => LocalisedText {
                en: "Treasure Hunt",
                ja: "トレジャーハント",
                de: "Schatzsuche",
                fr: "Chasse aux trésors",
            },
            Self::TheHunt => LocalisedText {
                en: "The Hunt",
                ja: "モブハント ",
                de: "Hohe Jagd",
                fr: "Contrats de chasse",
            },
            Self::GatheringForays => LocalisedText {
                en: "Gathering Forays",
                ja: "採集活動",
                de: "Sammeln",
                fr: "Récolte",
            },
            Self::DeepDungeons => LocalisedText {
                en: "Deep Dungeons",
                ja: "ディープダンジョン",
                de: "Tiefe Gewölbe",
                fr: "Donjons sans fond",
            },
            Self::FieldOperations => LocalisedText {
                en: "Field Operations",
                ja: "特殊フィールド探索",
                de: "Feldexkursion",
                fr: "Missions d'exploration",
            },
            Self::VariantAndCriterionDungeonFinder => LocalisedText {
                en: "V&C Dungeon Finder",
                ja: "特殊ダンジョン探索",
                de: "Gewölbesuche",
                fr: "Donjons spéciaux",
            },
            Self::None => LocalisedText {
                en: "None",
                ja: "設定なし",
                de: "Nicht festgelegt",
                fr: "Non spécifiée",
            },
        }
    }
}
