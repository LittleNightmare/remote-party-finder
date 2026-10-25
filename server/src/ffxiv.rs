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
    lfg_extension_content::LFG_EXTENSION_CONTENT,
    roulettes::ROULETTES,
    territory_names::TERRITORY_NAMES,
    treasure_maps::TREASURE_MAPS,
    worlds::WORLDS,
};

pub mod auto_translate;
pub mod duties;
pub mod jobs;
pub mod lfg_extension_content;
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

/// Lightweight validation: Check if duty/category/duty_type combination is valid.
/// Returns Ok(()) if valid, Err(String) with base error message if invalid.
pub fn is_valid_duty_combination(duty_type: DutyType, category: DutyCategory, duty: u16) -> Result<(), String> {
    match (duty_type, category) {
        // Fate: always valid
        (DutyType::Other, DutyCategory::Fate) => Ok(()),

        // TheHunt: always valid
        (DutyType::Other, DutyCategory::TheHunt) => Ok(()),

        // None: duty must be 0
        (_, DutyCategory::None) if duty == 0 => Ok(()),
        (_, DutyCategory::None) => Err(format!("None category requires duty=0, got duty={}", duty)),

        // Normal: duty > 0 (allow unrecorded new duties)
        (DutyType::Normal, _) if duty > 0 => Ok(()),
        (DutyType::Normal, _) => Err(format!("Normal type requires duty>0, got duty={}", duty)),

        // Roulette: duty must exist in ROULETTES
        (DutyType::Roulette, _) => {
            match roulette(u32::from(duty)) {
                Some(_) => Ok(()),
                None => Err(format!("Roulette duty {} not found in ROULETTES", duty)),
            }
        }

        // GoldSaucer / DeepDungeon: duty is a 1-based LFGExtensionContent row id
        // (group 3 = gold saucer list, group 4 = deep dungeon list, mirroring the
        // client's selection checker); duty = 0 is what the client stores when it
        // fails to resolve a selection and keeps the category.
        // Derivation & re-verification playbook: README「PF 招募的分类与任务编号」.
        (_, DutyCategory::GoldSaucer) | (_, DutyCategory::DeepDungeon) => {
            let expected_group = if category == DutyCategory::GoldSaucer { 3 } else { 4 };
            match LFG_EXTENSION_CONTENT.get(&u32::from(duty)) {
                Some(entry) if entry.group == expected_group => Ok(()),
                _ if duty == 0 => Ok(()),
                _ => Err(format!(
                    "{:?} requires a known PF extension entry or duty=0, got duty={}",
                    category, duty
                )),
            }
        }

        // TreasureHunt: duty must exist in TREASURE_MAPS
        (_, DutyCategory::TreasureHunt) => {
            match crate::ffxiv::TREASURE_MAPS.get(&u32::from(duty)) {
                Some(_) => Ok(()),
                None => Err(format!("TreasureHunt duty {} not found in TREASURE_MAPS", duty)),
            }
        }

        // Other cases: invalid (use different prefix to distinguish from _duty_name_impl)
        _ => Err(format!("VALIDATION_FAILED: unknown type/category/duty: {:?}/{:?}/{}", duty_type, category, duty)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_and_names_blunderville_gold_saucer_listing() {
        assert_eq!(
            is_valid_duty_combination(DutyType::Other, DutyCategory::GoldSaucer, 27),
            Ok(())
        );
        assert_eq!(
            duty_name_simple(
                DutyType::Other,
                DutyCategory::GoldSaucer,
                27,
                Language::ChineseSimplified,
            ),
            "金碟巨豆中心"
        );
    }

    #[test]
    fn accepts_and_names_deep_dungeon_listing_from_lfg_extension_content_table() {
        // 死者宫殿: LFGExtensionContent row 28 (group 4), wire duty 29
        assert_eq!(
            is_valid_duty_combination(DutyType::Other, DutyCategory::DeepDungeon, 29),
            Ok(())
        );
        assert_eq!(
            duty_name_simple(
                DutyType::Other,
                DutyCategory::DeepDungeon,
                29,
                Language::ChineseSimplified,
            ),
            "死者宫殿 地下1～10层"
        );
    }

    #[test]
    fn accepts_unresolved_extension_duty() {
        // the client stores duty = 0 (keeping the category) when it fails to
        // resolve a selection, e.g. 深层迷宫 listings posted without one
        assert_eq!(
            is_valid_duty_combination(DutyType::Other, DutyCategory::DeepDungeon, 0),
            Ok(())
        );
        assert_eq!(
            is_valid_duty_combination(DutyType::Other, DutyCategory::GoldSaucer, 0),
            Ok(())
        );
        assert_eq!(
            duty_name_simple(
                DutyType::Other,
                DutyCategory::DeepDungeon,
                0,
                Language::ChineseSimplified,
            ),
            "深层迷宫"
        );
    }

    #[test]
    fn accepts_new_gold_saucer_entry_and_rejects_cross_group_duties() {
        // 魔光键影: LFGExtensionContent row 34 (group 3), wire duty 35
        assert_eq!(
            is_valid_duty_combination(DutyType::Other, DutyCategory::GoldSaucer, 35),
            Ok(())
        );
        // deep dungeon entries live in group 4 and must not match gold saucer,
        // and vice versa
        assert_eq!(
            is_valid_duty_combination(DutyType::Other, DutyCategory::GoldSaucer, 29),
            Err("GoldSaucer requires a known PF extension entry or duty=0, got duty=29".to_string())
        );
        assert_eq!(
            is_valid_duty_combination(DutyType::Other, DutyCategory::DeepDungeon, 35),
            Err("DeepDungeon requires a known PF extension entry or duty=0, got duty=35".to_string())
        );
        // values outside the sheet have no entry at all
        assert!(is_valid_duty_combination(DutyType::Other, DutyCategory::DeepDungeon, 1000).is_err());
    }
}

pub fn duty_name<'a>(
    duty_type: DutyType,
    category: DutyCategory,
    duty: u16,
    lang: Language,
    player_name: Option<&str>,
    world: Option<&str>,
    description: Option<&str>,
) -> Cow<'a, str> {
    _duty_name_impl(duty_type, category, duty, lang, player_name, world, description)
}

pub fn duty_name_simple<'a>(duty_type: DutyType, category: DutyCategory, duty: u16, lang: Language) -> Cow<'a, str> {
    _duty_name_impl(duty_type, category, duty, lang, None, None, None)
}

fn _duty_name_impl<'a>(
    duty_type: DutyType,
    category: DutyCategory,
    duty: u16,
    lang: Language,
    player_name: Option<&str>,
    world: Option<&str>,
    description: Option<&str>,
) -> Cow<'a, str> {
    match (duty_type, category) {
        (DutyType::Other, DutyCategory::Fate) => {
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
        (_, DutyCategory::None) if duty == 0 => return Cow::from(match lang {
            Language::English => "None",
            Language::Japanese => "設定なし",
            Language::German => "Nicht festgelegt",
            Language::French => "Non spécifiée",
            Language::ChineseSimplified => "无",
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
        (_, DutyCategory::GoldSaucer) if duty == 0 => return Cow::from(match lang {
            Language::English => "Gold Saucer",
            Language::Japanese => "ゴールドソーサー",
            Language::German => "Goldsaucer",
            Language::French => "Gold Saucer",
            Language::ChineseSimplified => "金碟游乐场",
        }),
        (_, DutyCategory::DeepDungeon) if duty == 0 => return Cow::from(match lang {
            Language::English => "Deep Dungeons",
            Language::Japanese => "ディープダンジョン",
            Language::German => "Tiefengewölbe",
            Language::French => "Donjons sans fond",
            Language::ChineseSimplified => "深层迷宫",
        }),
        // gold saucer / deep dungeon duties resolve through LFGExtensionContent
        // (wire duty = sheet row + 1); GATEs have no underlying duty, the client
        // shows a fixed addon label for them
        (_, DutyCategory::GoldSaucer | DutyCategory::DeepDungeon) => {
            if duty == 11 {
                return Cow::from(match lang {
                    Language::English => "GATEs",
                    Language::Japanese => "G.A.T.E.",
                    Language::German => "GATEs",
                    Language::French => "JACTA",
                    Language::ChineseSimplified => "机遇任务",
                });
            }
            if let Some(entry) = LFG_EXTENSION_CONTENT.get(&u32::from(duty)) {
                match entry.content {
                    lfg_extension_content::LfgContent::Regular(row) => {
                        if let Some(info) = crate::ffxiv::duty(row) {
                            return Cow::from(info.name.text(&lang));
                        }
                    }
                    lfg_extension_content::LfgContent::Roulette(row) => {
                        if let Some(info) = roulette(u32::from(row)) {
                            return Cow::from(info.name.text(&lang));
                        }
                    }
                    lfg_extension_content::LfgContent::Unresolved => {}
                }
            }
        }
        (_, DutyCategory::TreasureHunt) => if let Some(name) = crate::ffxiv::TREASURE_MAPS.get(&u32::from(duty)) {
            return Cow::from(name.text(&lang));
        }
        _ => {}
    }

    // Build error message with context information
    let mut error_msg = format!("unknown type/category/duty: {:?}/{:?}/{}", duty_type, category, duty);
    
    if let Some(name) = player_name {
        error_msg.push_str(&format!(" | player: {}", name));
    }
    if let Some(w) = world {
        error_msg.push_str(&format!(" | world: {}", w));
    }
    if let Some(desc) = description {
        // Take first 5 characters (safe for UTF-8)
        let desc_preview: String = desc.chars().take(5).collect();
        error_msg.push_str(&format!(" | description: {}", desc_preview));
    }
    
    tracing::warn!("{}", error_msg);
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
            963 => DutyInfo {
                name: LocalisedText {
                    en: "The Singularity Reactor (Unreal)",
                    ja: "幻ナイツ・オブ・ラウンド討滅戦",
                    de: "Traumprüfung - Singularitäts-Reaktor",
                    fr: "Le Réacteur de singularité (irréel)",
                    zh: "圆桌骑士幻巧战",
                },
                high_end: true,
                content_kind: ContentKind::Trials,
            },
            1007 => DutyInfo {
                name: LocalisedText {
                    en: "The Jade Stoa (Unreal)",
                    ja: "幻白虎征魂戦",
                    de: "Traumprüfung - Byakko",
                    fr: "La Clairière de Jade (irréel)",
                    zh: "白虎幻巧战",
                },
                high_end: true,
                content_kind: ContentKind::Trials,
            },
            1029 => DutyInfo {
                name: LocalisedText {
                    en: "Hells' Kier (Unreal)",
                    ja: "幻朱雀征魂戦",
                    de: "Traumprüfung - Suzaku",
                    fr: "Le Nid des Lamentations (irréel)",
                    zh: "朱雀幻巧战",
                },
                high_end: true,
                content_kind: ContentKind::Trials,
            },
            1042 => DutyInfo {
                name: LocalisedText {
                    en: "The Wreath of Snakes (Unreal)",
                    ja: "幻青龍征魂戦",
                    de: "Traumprüfung - Seiryu",
                    fr: "L'Îlot des Amertumes (irréel)",
                    zh: "青龙幻巧战",
                },
                high_end: true,
                content_kind: ContentKind::Trials,
            },
            1067 => DutyInfo {
            name: LocalisedText {
                en: "Tsukuyomi's Pain (Unreal)",
                ja: "幻ツクヨミ討滅戦",
                de: "Traumprüfung - Tsukuyomi",
                fr: "Castrum Fluminis (irréel)",
                zh: "月读幻巧战",
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
