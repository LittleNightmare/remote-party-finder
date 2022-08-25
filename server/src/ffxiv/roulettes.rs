use std::collections::HashMap;
use super::LocalisedText;

#[derive(Debug)]
pub struct RouletteInfo {
    pub name: LocalisedText,
    pub pvp: bool,
}

lazy_static::lazy_static! {
    pub static ref ROULETTES: HashMap<u32, RouletteInfo> = maplit::hashmap! {
        1 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Leveling",
                ja: "コンテンツルーレット：レべリング",
                de: "Zufallsinhalt: Stufensteigerung",
                fr: "Mission aléatoire : gain de niveaux",
            },
            pvp: false,
        },
        2 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Level 50/60/70/80 Dungeons",
                ja: "コンテンツルーレット：レベル50・60・70・80ダンジョン",
                de: "Zufallsinhalt: Stufe 50/60/70/80",
                fr: "Mission aléatoire : donjons nv 50/60/70/80",
            },
            pvp: false,
        },
        3 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Main Scenario",
                ja: "コンテンツルーレット：メインクエスト",
                de: "Zufallsinhalt: Hauptszenario",
                fr: "Mission aléatoire : épopée",
            },
            pvp: false,
        },
        4 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Guildhests",
                ja: "コンテンツルーレット：ギルドオーダー",
                de: "Zufallsinhalt: Gildengeheiß",
                fr: "Mission aléatoire : opérations de guilde",
            },
            pvp: false,
        },
        5 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Expert",
                ja: "コンテンツルーレット：エキスパート",
                de: "Zufallsinhalt: Experte",
                fr: "Mission aléatoire : expert",
            },
            pvp: false,
        },
        6 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Trials",
                ja: "コンテンツルーレット：討伐・討滅戦",
                de: "Zufallsinhalt: Prüfung",
                fr: "Mission aléatoire : défis",
            },
            pvp: false,
        },
        7 => RouletteInfo {
            name: LocalisedText {
                en: "Daily Challenge: Frontline",
                ja: "デイリーチャレンジ：フロントライン",
                de: "Tagesherausforderung: PvP-Front",
                fr: "Challenge quotidien : Front",
            },
            pvp: false,
        },
        8 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Level 90 Dungeons",
                ja: "コンテンツルーレット：レベル90ダンジョン",
                de: "Zufallsinhalt: Stufe 90",
                fr: "Mission aléatoire : donjons nv 90",
            },
            pvp: false,
        },
        9 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Mentor",
                ja: "コンテンツルーレット：メンター",
                de: "Zufallsinhalt: Mentor",
                fr: "Mission aléatoire : mentor",
            },
            pvp: false,
        },
        15 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Alliance Raids",
                ja: "コンテンツルーレット：アライアンスレイド",
                de: "Zufallsinhalt: Allianz-Raid",
                fr: "Mission aléatoire : raids en alliance",
            },
            pvp: false,
        },
        17 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Normal Raids",
                ja: "コンテンツルーレット：ノーマルレイド",
                de: "Zufallsinhalt: Normaler Raid",
                fr: "Mission aléatoire : raids normaux",
            },
            pvp: false,
        },
        18 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road",
                ja: "チョコボレース：サゴリーロード",
                de: "Chocobo-Rennen: Sagolii-Straße",
                fr: "Course de chocobos : Route de Sagolii",
            },
            pvp: false,
        },
        19 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol",
                ja: "チョコボレース：コスタ・デル・ソル",
                de: "Chocobo-Rennen: Sonnenküste",
                fr: "Course de chocobos : Costa del Sol",
            },
            pvp: false,
        },
        20 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths",
                ja: "チョコボレース：トランキルパス",
                de: "Chocobo-Rennen: Pfad der Seelenruhe",
                fr: "Course de chocobos : Sentes tranquilles",
            },
            pvp: false,
        },
        21 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
            },
            pvp: false,
        },
        22 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road (No Rewards)",
                ja: "チョコボレース：サゴリーロード (報酬なし)",
                de: "Chocobo-Rennen: Sagolii-Straße (keine Belohnung)",
                fr: "Course de chocobos : Route de Sagolii (sans récompense)",
            },
            pvp: false,
        },
        23 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol (No Rewards)",
                ja: "チョコボレース：コスタ・デル・ソル (報酬なし)",
                de: "Chocobo-Rennen: Sonnenküste (keine Belohnung)",
                fr: "Course de chocobos : Costa del Sol (sans récompense)",
            },
            pvp: false,
        },
        24 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths (No Rewards)",
                ja: "チョコボレース：トランキルパス (報酬なし)",
                de: "Chocobo-Rennen: Pfad der Seelenruhe (keine Belohnung)",
                fr: "Course de chocobos : Sentes tranquilles (sans récompense)",
            },
            pvp: false,
        },
        25 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random (No Rewards)",
                ja: "チョコボレース：コースルーレット (報酬なし)",
                de: "Chocobo-Rennen: Zufallsstrecke (keine Belohnung)",
                fr: "Course de chocobos : aléatoire (sans récompense)",
            },
            pvp: false,
        },
        26 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
            },
            pvp: false,
        },
        27 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
            },
            pvp: false,
        },
        28 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
            },
            pvp: false,
        },
        29 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
            },
            pvp: false,
        },
        30 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
            },
            pvp: false,
        },
        31 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
            },
            pvp: false,
        },
        32 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
            },
            pvp: false,
        },
        33 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
            },
            pvp: false,
        },
        34 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
            },
            pvp: false,
        },
        35 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
            },
            pvp: false,
        },
        36 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
            },
            pvp: false,
        },
        37 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
            },
            pvp: false,
        },
        38 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
            },
            pvp: false,
        },
        40 => RouletteInfo {
            name: LocalisedText {
                en: "Crystalline Conflict (Casual Match)",
                ja: "クリスタルコンフリクト(カジュアルマッチ)",
                de: "Crystalline Conflict: Freies Spiel",
                fr: "Crystalline Conflict (partie non classée)",
            },
            pvp: true,
        },
        41 => RouletteInfo {
            name: LocalisedText {
                en: "Crystalline Conflict (Ranked Match)",
                ja: "クリスタルコンフリクト(ランクマッチ)",
                de: "Crystalline Conflict: Gewertetes Spiel",
                fr: "Crystalline Conflict (partie classée)",
            },
            pvp: true,
        },
    };
}
