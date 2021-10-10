use std::collections::HashMap;
use super::LocalisedText;

lazy_static::lazy_static! {
    pub static ref TREASURE_MAPS: HashMap<u32, LocalisedText> = maplit::hashmap! {
        0 => LocalisedText {
            en: "All Levels",
            ja: "レベルを指定しない",
            de: "Jede Stufe",
            fr: "Tous niveaux",
        },
        1 => LocalisedText {
            en: "Leather Treasure Map",
            ja: "古ぼけた地図G1",
            de: "Leder-Schatzkarte",
            fr: "Carte au trésor en cuir",
        },
        2 => LocalisedText {
            en: "Leather Treasure Map",
            ja: "古ぼけた地図G1",
            de: "Leder-Schatzkarte",
            fr: "Carte au trésor en cuir",
        },
        3 => LocalisedText {
            en: "Goatskin Treasure Map",
            ja: "古ぼけた地図G2",
            de: "Steinbockleder-Schatzkarte",
            fr: "Carte au trésor en peau de bouquetin",
        },
        4 => LocalisedText {
            en: "Toadskin Treasure Map",
            ja: "古ぼけた地図G3",
            de: "Krötenleder-Schatzkarte",
            fr: "Carte au trésor en peau de crapaud",
        },
        5 => LocalisedText {
            en: "Boarskin Treasure Map",
            ja: "古ぼけた地図G4",
            de: "Keilerleder-Schatzkarte",
            fr: "Carte au trésor en peau de sanglier",
        },
        6 => LocalisedText {
            en: "Peisteskin Treasure Map",
            ja: "古ぼけた地図G5",
            de: "Basiliskenleder-Schatzkarte",
            fr: "Carte au trésor en peau de peiste",
        },
        7 => LocalisedText {
            en: "Leather Buried Treasure Map",
            ja: "隠された地図G1",
            de: "Kryptische Karte",
            fr: "Carte au trésor secrète en cuir",
        },
        8 => LocalisedText {
            en: "Archaeoskin Treasure Map",
            ja: "古ぼけた地図G6",
            de: "Archaeoleder-Schatzkarte",
            fr: "Carte au trésor en peau d'archéornis",
        },
        9 => LocalisedText {
            en: "Wyvernskin Treasure Map",
            ja: "古ぼけた地図G7",
            de: "Wyvernleder-Schatzkarte",
            fr: "Carte au trésor en peau de wyverne",
        },
        10 => LocalisedText {
            en: "Dragonskin Treasure Map",
            ja: "古ぼけた地図G8",
            de: "Drachenleder-Schatzkarte",
            fr: "Carte au trésor en peau de dragon",
        },
        11 => LocalisedText {
            en: "Gaganaskin Treasure Map",
            ja: "古ぼけた地図G9",
            de: "Gaganaleder-Schatzkarte",
            fr: "Carte au trésor en peau de gagana",
        },
        12 => LocalisedText {
            en: "Gazelleskin Treasure Map",
            ja: "古ぼけた地図G10",
            de: "Gazellenleder-Schatzkarte",
            fr: "Carte au trésor en peau de gazelle",
        },
        13 => LocalisedText {
            en: "Seemingly Special Treasure Map",
            ja: "古ぼけた地図S1",
            de: "Exotenleder-Schatzkarte",
            fr: "Carte au trésor inhabituelle I",
        },
        14 => LocalisedText {
            en: "Gliderskin Treasure Map",
            ja: "古ぼけた地図G11",
            de: "Smilodonleder-Schatzkarte",
            fr: "Carte au trésor en peau de smilodon",
        },
        15 => LocalisedText {
            en: "Zonureskin Treasure Map",
            ja: "古ぼけた地図G12",
            de: "Glaucusleder-Schatzkarte",
            fr: "Carte au trésor en peau de glaucus",
        },
        16 => LocalisedText {
            en: "Presumably Special Treasure Map",
            ja: "古ぼけた地図S2",
            de: "Mythenleder-Schatzkarte",
            fr: "Carte au trésor inhabituelle II",
        },
    };
}
