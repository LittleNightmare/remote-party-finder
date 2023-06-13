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
                chs: "随机任务：练级",
            },
            pvp: false,
        },
        2 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Level 50/60/70/80 Dungeons",
                ja: "コンテンツルーレット：レベル50・60・70・80ダンジョン",
                de: "Zufallsinhalt: Stufe 50/60/70/80",
                fr: "Mission aléatoire : donjons nv 50/60/70/80",
                chs: "随机任务：50级/60级/70级/80级",
            },
            pvp: false,
        },
        3 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Main Scenario",
                ja: "コンテンツルーレット：メインクエスト",
                de: "Zufallsinhalt: Hauptszenario",
                fr: "Mission aléatoire : épopée",
                chs: "随机任务：主线任务",
            },
            pvp: false,
        },
        4 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Guildhests",
                ja: "コンテンツルーレット：ギルドオーダー",
                de: "Zufallsinhalt: Gildengeheiß",
                fr: "Mission aléatoire : opérations de guilde",
                chs: "随机任务：行会令",
            },
            pvp: false,
        },
        5 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Expert",
                ja: "コンテンツルーレット：エキスパート",
                de: "Zufallsinhalt: Experte",
                fr: "Mission aléatoire : expert",
                chs: "随机任务：高难度",
            },
            pvp: false,
        },
        6 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Trials",
                ja: "コンテンツルーレット：討伐・討滅戦",
                de: "Zufallsinhalt: Prüfung",
                fr: "Mission aléatoire : défis",
                chs: "随机任务：讨伐歼灭战",
            },
            pvp: false,
        },
        7 => RouletteInfo {
            name: LocalisedText {
                en: "Daily Challenge: Frontline",
                ja: "デイリーチャレンジ：フロントライン",
                de: "Tagesherausforderung: PvP-Front",
                fr: "Challenge quotidien : Front",
                chs: "每日挑战：纷争前线",
            },
            pvp: true,
        },
        8 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Level 90 Dungeons",
                ja: "コンテンツルーレット：レベル90ダンジョン",
                de: "Zufallsinhalt: Stufe 90",
                fr: "Mission aléatoire : donjons nv 90",
                chs: "随机任务：90级",
            },
            pvp: false,
        },
        9 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Mentor",
                ja: "コンテンツルーレット：メンター",
                de: "Zufallsinhalt: Mentor",
                fr: "Mission aléatoire : mentor",
                chs: "随机任务：指导者",
            },
            pvp: false,
        },
        15 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Alliance Raids",
                ja: "コンテンツルーレット：アライアンスレイド",
                de: "Zufallsinhalt: Allianz-Raid",
                fr: "Mission aléatoire : raids en alliance",
                chs: "随机任务：团队任务",
            },
            pvp: false,
        },
        17 => RouletteInfo {
            name: LocalisedText {
                en: "Duty Roulette: Normal Raids",
                ja: "コンテンツルーレット：ノーマルレイド",
                de: "Zufallsinhalt: Normaler Raid",
                fr: "Mission aléatoire : raids normaux",
                chs: "随机任务：大型任务",
            },
            pvp: false,
        },
        18 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road",
                ja: "チョコボレース：サゴリーロード",
                de: "Chocobo-Rennen: Sagolii-Straße",
                fr: "Course de chocobos : Route de Sagolii",
                chs: "陆行鸟竞赛：荒野大道",
            },
            pvp: false,
        },
        19 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol",
                ja: "チョコボレース：コスタ・デル・ソル",
                de: "Chocobo-Rennen: Sonnenküste",
                fr: "Course de chocobos : Costa del Sol",
                chs: "陆行鸟竞赛：太阳海岸",
            },
            pvp: false,
        },
        20 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths",
                ja: "チョコボレース：トランキルパス",
                de: "Chocobo-Rennen: Pfad der Seelenruhe",
                fr: "Course de chocobos : Sentes tranquilles",
                chs: "陆行鸟竞赛：恬静小路",
            },
            pvp: false,
        },
        21 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
                chs: "陆行鸟竞赛：随机赛道",
            },
            pvp: false,
        },
        22 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road (No Rewards)",
                ja: "チョコボレース：サゴリーロード (報酬なし)",
                de: "Chocobo-Rennen: Sagolii-Straße (keine Belohnung)",
                fr: "Course de chocobos : Route de Sagolii (sans récompense)",
                chs: "陆行鸟竞赛：荒野大道（无报酬）",
            },
            pvp: false,
        },
        23 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol (No Rewards)",
                ja: "チョコボレース：コスタ・デル・ソル (報酬なし)",
                de: "Chocobo-Rennen: Sonnenküste (keine Belohnung)",
                fr: "Course de chocobos : Costa del Sol (sans récompense)",
                chs: "陆行鸟竞赛：太阳海岸（无报酬）",
            },
            pvp: false,
        },
        24 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths (No Rewards)",
                ja: "チョコボレース：トランキルパス (報酬なし)",
                de: "Chocobo-Rennen: Pfad der Seelenruhe (keine Belohnung)",
                fr: "Course de chocobos : Sentes tranquilles (sans récompense)",
                chs: "陆行鸟竞赛：恬静小路（无报酬）",
            },
            pvp: false,
        },
        25 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random (No Rewards)",
                ja: "チョコボレース：コースルーレット (報酬なし)",
                de: "Chocobo-Rennen: Zufallsstrecke (keine Belohnung)",
                fr: "Course de chocobos : aléatoire (sans récompense)",
                chs: "陆行鸟竞赛：随机赛道（无报酬）",
            },
            pvp: false,
        },
        26 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
                chs: "陆行鸟竞赛：随机赛道",
            },
            pvp: false,
        },
        27 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
                chs: "陆行鸟竞赛：随机赛道",
            },
            pvp: false,
        },
        28 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
                chs: "陆行鸟竞赛：随机赛道",
            },
            pvp: false,
        },
        29 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
                chs: "陆行鸟竞赛：随机赛道",
            },
            pvp: false,
        },
        30 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
                chs: "陆行鸟竞赛：随机赛道",
            },
            pvp: false,
        },
        31 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
                chs: "陆行鸟竞赛：随机赛道",
            },
            pvp: false,
        },
        32 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
                chs: "陆行鸟竞赛：随机赛道",
            },
            pvp: false,
        },
        33 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
                chs: "陆行鸟竞赛：随机赛道",
            },
            pvp: false,
        },
        34 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
                chs: "陆行鸟竞赛：随机赛道",
            },
            pvp: false,
        },
        35 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
                chs: "陆行鸟竞赛：随机赛道",
            },
            pvp: false,
        },
        36 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
                chs: "陆行鸟竞赛：随机赛道",
            },
            pvp: false,
        },
        37 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
                chs: "陆行鸟竞赛：随机赛道",
            },
            pvp: false,
        },
        38 => RouletteInfo {
            name: LocalisedText {
                en: "Chocobo Race: Random",
                ja: "チョコボレース：コースルーレット",
                de: "Chocobo-Rennen: Zufallsstrecke",
                fr: "Course de chocobos : aléatoire",
                chs: "陆行鸟竞赛：随机赛道",
            },
            pvp: false,
        },
        40 => RouletteInfo {
            name: LocalisedText {
                en: "Crystalline Conflict (Casual Match)",
                ja: "クリスタルコンフリクト(カジュアルマッチ)",
                de: "Crystalline Conflict: Freies Spiel",
                fr: "Crystalline Conflict (partie non classée)",
                chs: "水晶冲突（练习赛）",
            },
            pvp: true,
        },
        41 => RouletteInfo {
            name: LocalisedText {
                en: "Crystalline Conflict (Ranked Match)",
                ja: "クリスタルコンフリクト(ランクマッチ)",
                de: "Crystalline Conflict: Gewertetes Spiel",
                fr: "Crystalline Conflict (partie classée)",
                chs: "水晶冲突（段位赛）",
            },
            pvp: true,
        },
    };
}
