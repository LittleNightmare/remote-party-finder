use std::collections::HashMap;
use super::LocalisedText;

lazy_static::lazy_static! {
    pub static ref TREASURE_MAPS: HashMap<u32, LocalisedText> = maplit::hashmap! {
        0 => LocalisedText {
            en: "All Levels",
            ja: "レベルを指定しない",
            de: "Jede Stufe",
            fr: "Tous niveaux",
            zh: "所有等级",
        },
        1 => LocalisedText {
            en: "Leather Treasure Map",
            ja: "古ぼけた地図G1",
            de: "Leder-Schatzkarte",
            fr: "Carte au trésor en cuir",
            zh: "陈旧的鞣革地图",
        },
        2 => LocalisedText {
            en: "Leather Treasure Map",
            ja: "古ぼけた地図G1",
            de: "Leder-Schatzkarte",
            fr: "Carte au trésor en cuir",
            zh: "陈旧的鞣革地图",
        },
        3 => LocalisedText {
            en: "Goatskin Treasure Map",
            ja: "古ぼけた地図G2",
            de: "Steinbockleder-Schatzkarte",
            fr: "Carte au trésor en peau de bouquetin",
            zh: "陈旧的山羊革地图",
        },
        4 => LocalisedText {
            en: "Toadskin Treasure Map",
            ja: "古ぼけた地図G3",
            de: "Krötenleder-Schatzkarte",
            fr: "Carte au trésor en peau de crapaud",
            zh: "陈旧的巨蟾蜍革地图",
        },
        5 => LocalisedText {
            en: "Boarskin Treasure Map",
            ja: "古ぼけた地図G4",
            de: "Keilerleder-Schatzkarte",
            fr: "Carte au trésor en peau de sanglier",
            zh: "陈旧的野猪革地图",
        },
        6 => LocalisedText {
            en: "Peisteskin Treasure Map",
            ja: "古ぼけた地図G5",
            de: "Basiliskenleder-Schatzkarte",
            fr: "Carte au trésor en peau de peiste",
            zh: "陈旧的毒蜥蜴革地图",
        },
        7 => LocalisedText {
            en: "Leather Buried Treasure Map",
            ja: "隠された地図G1",
            de: "Kryptische Karte",
            fr: "Carte au trésor secrète en cuir",
            zh: "鞣革制的隐藏地图",
        },
        8 => LocalisedText {
            en: "Archaeoskin Treasure Map",
            ja: "古ぼけた地図G6",
            de: "Archaeoleder-Schatzkarte",
            fr: "Carte au trésor en peau d'archéornis",
            zh: "陈旧的古鸟革地图",
        },
        9 => LocalisedText {
            en: "Wyvernskin Treasure Map",
            ja: "古ぼけた地図G7",
            de: "Wyvernleder-Schatzkarte",
            fr: "Carte au trésor en peau de wyverne",
            zh: "陈旧的飞龙革地图",
        },
        10 => LocalisedText {
            en: "Dragonskin Treasure Map",
            ja: "古ぼけた地図G8",
            de: "Drachenleder-Schatzkarte",
            fr: "Carte au trésor en peau de dragon",
            zh: "陈旧的巨龙革地图",
        },
        11 => LocalisedText {
            en: "Gaganaskin Treasure Map",
            ja: "古ぼけた地図G9",
            de: "Gaganaleder-Schatzkarte",
            fr: "Carte au trésor en peau de gagana",
            zh: "陈旧的迦迦纳怪鸟革地图",
        },
        12 => LocalisedText {
            en: "Gazelleskin Treasure Map",
            ja: "古ぼけた地図G10",
            de: "Gazellenleder-Schatzkarte",
            fr: "Carte au trésor en peau de gazelle",
            zh: "陈旧的瞪羚革地图",
        },
        13 => LocalisedText {
            en: "Seemingly Special Treasure Map",
            ja: "古ぼけた地図S1",
            de: "Exotenleder-Schatzkarte",
            fr: "Carte au trésor inhabituelle I",
            zh: "陈旧的特殊地图1",
        },
        14 => LocalisedText {
            en: "Gliderskin Treasure Map",
            ja: "古ぼけた地図G11",
            de: "Smilodonleder-Schatzkarte",
            fr: "Carte au trésor en peau de smilodon",
            zh: "陈旧的绿飘龙革地图",
        },
        15 => LocalisedText {
            en: "Zonureskin Treasure Map",
            ja: "古ぼけた地図G12",
            de: "Glaucusleder-Schatzkarte",
            fr: "Carte au trésor en peau de glaucus",
            zh: "陈旧的缠尾蛟革地图",
        },
        16 => LocalisedText {
            en: "Ostensibly Special Treasure Map",
            ja: "古ぼけた地図S2",
            de: "Mythenleder-Schatzkarte",
            fr: "Carte au trésor inhabituelle II",
            zh: "陈旧的特殊地图2",
        },
        17 => LocalisedText {
            en: "Saigaskin Treasure Map",
            ja: "古ぼけた地図G13",
            de: "Gajaleder-Schatzkarte",
            fr: "Carte au trésor en peau de gaja",
            zh: "陈旧的赛加羚羊革地图",
        },
        18 => LocalisedText {
            en: "Kumbhiraskin Treasure Map",
            ja: "古ぼけた地図G14",
            de: "Kumbhilaleder-Schatzkarte",
            fr: "Carte au trésor en peau de kumbhira",
            zh: "陈旧的金毗罗鳄革地图",
        },
        19 => LocalisedText {
            en: "Ophiotauroskin Treasure Map",
            ja: "古ぼけた地図G15",
            de: "Ophiotaurosleder-Schatzkarte",
            fr: "Carte au trésor en peau d'ophiotauros",
            zh: "陈旧的蛇牛革地图",
        },
        20 => LocalisedText {
            en: "Potentially Special Treasure Map",
            ja: "古ぼけた地図S3",
            de: "Legendenleder-Schatzkarte",
            fr: "Carte au trésor inhabituelle III",
            zh: "陈旧的特殊地图3",
        },
        21 => LocalisedText {
            en: "Conceivably Special Treasure Map",
            ja: "古ぼけた地図S4",
            de: "Sagenleder-Schatzkarte",
            fr: "Carte au trésor inhabituelle IV",
            zh: "陈旧的特殊地图4",
        },
        22 => LocalisedText {
            en: "Loboskin Treasure Map",
            ja: "古ぼけた地図G16",
            de: "Schakalleder-Schatzkarte",
            fr: "Carte au trésor en peau de loup argenté",
            zh: "陈旧的银狼革地图",
        },
        23 => LocalisedText {
            en: "Br'aaxskin Treasure Map",
            ja: "古ぼけた地図G17",
            de: "Br'aaxleder-Schatzkarte",
            fr: "Carte au trésor en peau de br'aax",
            zh: "陈旧的狞豹革地图",
        },
    };
}
