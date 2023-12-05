use std::{
    cmp::Ordering,
    str::FromStr,
};
use std::borrow::Cow;

use crate::listing::{DutyCategory, DutyType};

pub use self::{
    auto_translate::AUTO_TRANSLATE,
    duties::DUTIES,
    jobs::JOBS,
    roulettes::ROULETTES,
    territory_names::TERRITORY_NAMES,
    treasure_maps::TREASURE_MAPS,
    worlds::WORLDS,
};

pub mod auto_translate;
pub mod duties;
pub mod jobs;
pub mod roulettes;
pub mod territory_names;
pub mod treasure_maps;
pub mod worlds;

#[derive(Debug, Copy, Clone)]
pub enum Language {
    English,
    Japanese,
    German,
    French,
    ChineseSimplified,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Japanese => "ja",
            Self::German => "de",
            Self::French => "fr",
            Self::ChineseSimplified => "zh",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::English => "english",
            Self::Japanese => "日本語",
            Self::German => "deutsch",
            Self::French => "français",
            Self::ChineseSimplified => "简体中文",
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
                "zh" => return Self::ChineseSimplified,
                _ => {}
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
    pub zh: &'static str,
}

impl LocalisedText {
    pub fn text(&self, lang: &Language) -> &'static str {
        match lang {
            Language::English => self.en,
            Language::Japanese => self.ja,
            Language::German => self.de,
            Language::French => self.fr,
            Language::ChineseSimplified => self.zh,
        }
    }
}

pub fn duty(duty: u32) -> Option<&'static duties::DutyInfo> {
    crate::ffxiv::DUTIES.get(&duty)
        .or_else(|| old::OLD_DUTIES.get(&duty))
}

pub fn roulette(roulette: u32) -> Option<&'static roulettes::RouletteInfo> {
    crate::ffxiv::ROULETTES.get(&roulette)
        .or_else(|| old::OLD_ROULETTES.get(&roulette))
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
            Language::ChineseSimplified => "怪物狩猎",
        }),
        (DutyType::Other, DutyCategory::Duty) if duty == 0 => return Cow::from(match lang {
            Language::English => "None",
            Language::Japanese => "設定なし",
            Language::German => "Nicht festgelegt",
            Language::French => "Non spécifiée",
            Language::ChineseSimplified => "无",
        }),
        (DutyType::Other, DutyCategory::DeepDungeons) if duty == 1 => return Cow::from(match lang {
            Language::English => "The Palace of the Dead",
            Language::Japanese => "死者の宮殿",
            Language::German => "Palast der Toten",
            Language::French => "Palais des morts",
            Language::ChineseSimplified => "死者宫殿",
        }),
        (DutyType::Other, DutyCategory::DeepDungeons) if duty == 2 => return Cow::from(match lang {
            Language::English => "Heaven-on-High",
            Language::Japanese => "アメノミハシラ",
            Language::German => "Himmelssäule",
            Language::French => "Pilier des Cieux",
            Language::ChineseSimplified => "天之御柱",
        }),
        (DutyType::Other, DutyCategory::DeepDungeons) if duty == 3 => return Cow::from(match lang {
            Language::English => "Eureka Orthos",
            Language::Japanese => "オルト・エウレカ",
            Language::German => "Eureka Orthos",
            Language::French => "Eurêka Orthos",
            Language::ChineseSimplified => "正统优雷卡",
        }),
        (DutyType::Normal, _) => {
            if let Some(info) = crate::ffxiv::duty(u32::from(duty)) {
                return Cow::from(info.name.text(&lang));
            }
        }
        (DutyType::Roulette, _) => {
            if let Some(info) = roulette(u32::from(duty)) {
                return Cow::from(info.name.text(&lang));
            }
        }
        (_, DutyCategory::QuestBattles) => return Cow::from(match lang {
            Language::English => "Quest Battles",
            Language::Japanese => "クエストバトル",
            Language::German => "Auftragskampf",
            Language::French => "Batailles de quête",
            Language::ChineseSimplified => "任务战斗",
        }),
        (_, DutyCategory::TreasureHunt) => if let Some(name) = crate::ffxiv::TREASURE_MAPS.get(&u32::from(duty)) {
            return Cow::from(name.text(&lang));
        }
        _ => {}
    }

    eprintln!("unknown type/category/duty: {:?}/{:?}/{}", duty_type, category, duty);
    Cow::from(format!("{:?}", category))
}

mod old {
    use std::collections::HashMap;

    use crate::ffxiv::{
        duties::{ContentKind, DutyInfo},
        LocalisedText,
        roulettes::RouletteInfo,
    };

    lazy_static::lazy_static! {
        pub static ref OLD_DUTIES: HashMap<u32, DutyInfo> = maplit::hashmap! {
            62 => DutyInfo {
                name: LocalisedText {
                    en: "Cape Westwind",
                    ja: "リットアティン強襲戦",
                    de: "Kap Westwind",
                    fr: "Le Cap Vendouest",
                    zh: "里塔提恩强攻战"
                },
                high_end: false,
                content_kind: ContentKind::Trials,
            },
            83 => DutyInfo {
                name: LocalisedText {
                    en: "The Steps of Faith",
                    ja: "皇都イシュガルド防衛戦",
                    de: "Der Schicksalsweg",
                    fr: "Le Siège de la sainte Cité d'Ishgard",
                    zh: "皇都伊修加德保卫战"
                },
                high_end: false,
                content_kind: ContentKind::Trials,
            },
            143 => DutyInfo {
                name: LocalisedText {
                    en: "The Feast (4 on 4 - Training)",
                    ja: "ザ・フィースト (4対4 / カジュアルマッチ)",
                    de: "The Feast (4 gegen 4, Übungskampf)",
                    fr: "The Feast (4x4/entraînement)",
                    zh: "群狼盛宴（4v4 训练赛）"
                },
                high_end: false,
                content_kind: ContentKind::PvP,
            },
            145 => DutyInfo {
                name: LocalisedText {
                    en: "The Feast (4 on 4 - Ranked)",
                    ja: "ザ・フィースト (4対4 / ランクマッチ)",
                    de: "The Feast (4 gegen 4, gewertet)",
                    fr: "The Feast (4x4/classé)",
                    zh: "群狼盛宴（4v4 段位赛）"
                },
                high_end: false,
                content_kind: ContentKind::PvP,
            },
            201 => DutyInfo {
                name: LocalisedText {
                    en: "The Feast (Custom Match - Feasting Grounds)",
                    ja: "ザ・フィースト (ウルヴズジェイル演習場：カスタムマッチ）",
                    de: "The Feast (Wolfshöhle: Schaukampf)",
                    fr: "The Feast (personnalisé/Festin des loups)",
                    zh: "群狼盛宴（4v4 自定义赛）"
                },
                high_end: false,
                content_kind: ContentKind::PvP,
            },
            228 => DutyInfo {
                name: LocalisedText {
                    en: "The Feast (4 on 4 - Training)",
                    ja: "ザ・フィースト (4対4 / カジュアルマッチ)",
                    de: "The Feast (4 gegen 4, Übungskampf)",
                    fr: "The Feast (4x4/entraînement)",
                    zh: "群狼盛宴（4v4 训练赛）"
                },
                high_end: false,
                content_kind: ContentKind::PvP,
            },
            230 => DutyInfo {
                name: LocalisedText {
                    en: "The Feast (4 on 4 - Ranked)",
                    ja: "ザ・フィースト (4対4 / ランクマッチ)",
                    de: "The Feast (4 gegen 4, gewertet)",
                    fr: "The Feast (4x4/classé)",
                    zh: "群狼盛宴（4v4 段位赛）"
                },
                high_end: false,
                content_kind: ContentKind::PvP,
            },
            233 => DutyInfo {
                name: LocalisedText {
                    en: "The Feast (Custom Match - Lichenweed)",
                    ja: "ザ・フィースト (ライケンウィード演習場：カスタムマッチ）",
                    de: "The Feast (Flechtenhain: Schaukampf)",
                    fr: "The Feast (personnalisé/Pré-de-lichen)",
                    zh: "群狼盛宴（4v4 自定义赛）"
                },
                high_end: false,
                content_kind: ContentKind::PvP,
            },
            476 => DutyInfo {
                name: LocalisedText {
                    en: "The Feast (Team Ranked)",
                    ja: "ザ・フィースト (チーム用ランクマッチ)",
                    de: "The Feast (Team, gewertet)",
                    fr: "The Feast (classé/équipe JcJ)",
                    zh: "群狼盛宴（4v4 团队段位赛）"
                },
                high_end: false,
                content_kind: ContentKind::PvP,
            },
            478 => DutyInfo {
                name: LocalisedText {
                    en: "The Feast (Ranked)",
                    ja: "ザ・フィースト (ランクマッチ)",
                    de: "The Feast (gewertet)",
                    fr: "The Feast (classé)",
                    zh: "群狼盛宴（段位赛）"
                },
                high_end: false,
                content_kind: ContentKind::PvP,
            },
            479 => DutyInfo {
                name: LocalisedText {
                    en: "The Feast (Training)",
                    ja: "ザ・フィースト (カジュアルマッチ)",
                    de: "The Feast (Übungskampf)",
                    fr: "The Feast (entraînement)",
                    zh: "群狼盛宴（训练赛）"
                },
                high_end: false,
                content_kind: ContentKind::PvP,
            },
            480 => DutyInfo {
                name: LocalisedText {
                    en: "The Feast (Custom Match - Crystal Tower)",
                    ja: "ザ・フィースト (クリスタルタワー演習場：カスタムマッチ）",
                    de: "The Feast (Kristallturm-Arena: Schaukampf)",
                    fr: "The Feast (personnalisé/Tour de Cristal)",
                    zh: "群狼盛宴（自定义赛）"
                },
                high_end: false,
                content_kind: ContentKind::PvP,
            },
            580 => DutyInfo {
                name: LocalisedText {
                    en: "The Feast (Team Custom Match - Crystal Tower)",
                    ja: "ザ・フィースト (クリスタルタワー演習場：チーム用カスタムマッチ)",
                    de: "The Feast (Kristallturm-Arena: Team-Schaukampf) ",
                    fr: "The Feast (personnalisé/équipe JcJ/Tour de Cristal)",
                    zh: "群狼盛宴（团队自定义赛）"
                },
                high_end: false,
                content_kind: ContentKind::PvP,
            },
            776 => DutyInfo {
                name: LocalisedText {
                    en: "The Whorleater (Unreal)",
                    ja: "幻リヴァイアサン討滅戦",
                    de: "Traumprüfung - Leviathan",
                    fr: "Le Briseur de marées (irréel)",
                    zh: "利维亚桑幻巧战"
                },
                high_end: true,
                content_kind: ContentKind::Trials,
            },
            821 => DutyInfo {
                name: LocalisedText {
                    en: "Ultima's Bane (Unreal)",
                    ja: "幻アルテマウェポン破壊作戦",
                    de: "Traumprüfung - Ultima",
                    fr: "Le fléau d'Ultima (irréel)",
                    zh: "究极神兵幻巧战"
                },
                high_end: true,
                content_kind: ContentKind::Trials,
            },
            875 => DutyInfo {
                name: LocalisedText {
                    en: "Containment Bay S1T7 (Unreal)",
                    ja: "幻魔神セフィロト討滅戦",
                    de: "Traumprüfung - Sephirot",
                    fr: "Unité de contention S1P7 (irréel)",
                    zh: "萨菲洛特幻巧战"
                },
                high_end: true,
                content_kind: ContentKind::Trials,
            },
            926 => DutyInfo {
                name: LocalisedText {
                    en: "Containment Bay P1T6 (Unreal)",
                    ja: "幻女神ソフィア討滅戦",
                    de: "Traumprüfung - Sophia",
                    fr: "Unité de contention P1P6 (irréel)",
                    zh: "索菲娅幻巧战"
                },
                high_end: true,
                content_kind: ContentKind::Trials,
            },
            951 => DutyInfo {
                name: LocalisedText {
                    en: "Containment Bay Z1T9 (Unreal)",
                    ja: "幻鬼神ズルワーン討滅戦",
                    de: "Traumprüfung - Zurvan",
                    fr: "Unité de contention Z1P9 (irréel)",
                    zh: "祖尔宛幻巧战",
                },
                high_end: true,
                content_kind: ContentKind::Trials,
            },
        };

        pub static ref OLD_ROULETTES: HashMap<u32, RouletteInfo> = maplit::hashmap! {
            11 => RouletteInfo {
                name: LocalisedText {
                    en: "The Feast (Training Match)",
                    ja: "ザ・フィースト (カジュアルマッチ)",
                    de: "The Feast (Übungskampf)",
                    fr: "The Feast (entraînement)",
                    zh: "群狼盛宴（训练赛）"
                },
                pvp: true,
            },
            13 => RouletteInfo {
                name: LocalisedText {
                    en: "The Feast (Ranked Match)",
                    ja: "ザ・フィースト (ランクマッチ)",
                    de: "The Feast (gewertet)",
                    fr: "The Feast (classé)",
                    zh: "群狼盛宴（段位赛）"
                },
                pvp: true,
            },
            16 => RouletteInfo {
                name: LocalisedText {
                    en: "The Feast (Team Ranked Match)",
                    ja: "ザ・フィースト (チーム用ランクマッチ)",
                    de: "The Feast (Team, gewertet)",
                    fr: "The Feast (classé/équipe JcJ)",
                    zh: "群狼盛宴（团队段位赛）"
                },
                pvp: true,
            },
        };
    }
}
