use std::collections::HashMap;
use super::LocalisedText;

lazy_static::lazy_static! {
    pub static ref TREASURE_MAPS: HashMap<u32, LocalisedText> = maplit::hashmap! {
        0 => LocalisedText {
            en: "All Levels",
            ja: "All Levels",
            de: "All Levels",
            fr: "All Levels",
        },
        1 => LocalisedText {
            en: "Leather Treasure Map",
            ja: "Leather Treasure Map",
            de: "Leather Treasure Map",
            fr: "Leather Treasure Map",
        },
        2 => LocalisedText {
            en: "Leather Treasure Map",
            ja: "Leather Treasure Map",
            de: "Leather Treasure Map",
            fr: "Leather Treasure Map",
        },
        3 => LocalisedText {
            en: "Goatskin Treasure Map",
            ja: "Goatskin Treasure Map",
            de: "Goatskin Treasure Map",
            fr: "Goatskin Treasure Map",
        },
        4 => LocalisedText {
            en: "Toadskin Treasure Map",
            ja: "Toadskin Treasure Map",
            de: "Toadskin Treasure Map",
            fr: "Toadskin Treasure Map",
        },
        5 => LocalisedText {
            en: "Boarskin Treasure Map",
            ja: "Boarskin Treasure Map",
            de: "Boarskin Treasure Map",
            fr: "Boarskin Treasure Map",
        },
        6 => LocalisedText {
            en: "Peisteskin Treasure Map",
            ja: "Peisteskin Treasure Map",
            de: "Peisteskin Treasure Map",
            fr: "Peisteskin Treasure Map",
        },
        7 => LocalisedText {
            en: "Leather Buried Treasure Map",
            ja: "Leather Buried Treasure Map",
            de: "Leather Buried Treasure Map",
            fr: "Leather Buried Treasure Map",
        },
        8 => LocalisedText {
            en: "Archaeoskin Treasure Map",
            ja: "Archaeoskin Treasure Map",
            de: "Archaeoskin Treasure Map",
            fr: "Archaeoskin Treasure Map",
        },
        9 => LocalisedText {
            en: "Wyvernskin Treasure Map",
            ja: "Wyvernskin Treasure Map",
            de: "Wyvernskin Treasure Map",
            fr: "Wyvernskin Treasure Map",
        },
        10 => LocalisedText {
            en: "Dragonskin Treasure Map",
            ja: "Dragonskin Treasure Map",
            de: "Dragonskin Treasure Map",
            fr: "Dragonskin Treasure Map",
        },
        11 => LocalisedText {
            en: "Gaganaskin Treasure Map",
            ja: "Gaganaskin Treasure Map",
            de: "Gaganaskin Treasure Map",
            fr: "Gaganaskin Treasure Map",
        },
        12 => LocalisedText {
            en: "Gazelleskin Treasure Map",
            ja: "Gazelleskin Treasure Map",
            de: "Gazelleskin Treasure Map",
            fr: "Gazelleskin Treasure Map",
        },
        13 => LocalisedText {
            en: "Seemingly Special Treasure Map",
            ja: "Seemingly Special Treasure Map",
            de: "Seemingly Special Treasure Map",
            fr: "Seemingly Special Treasure Map",
        },
        14 => LocalisedText {
            en: "Gliderskin Treasure Map",
            ja: "Gliderskin Treasure Map",
            de: "Gliderskin Treasure Map",
            fr: "Gliderskin Treasure Map",
        },
        15 => LocalisedText {
            en: "Zonureskin Treasure Map",
            ja: "Zonureskin Treasure Map",
            de: "Zonureskin Treasure Map",
            fr: "Zonureskin Treasure Map",
        },
        16 => LocalisedText {
            en: "Presumably Special Treasure Map",
            ja: "Presumably Special Treasure Map",
            de: "Presumably Special Treasure Map",
            fr: "Presumably Special Treasure Map",
        },
    };
}
