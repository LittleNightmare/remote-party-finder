use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref TREASURE_MAPS: HashMap<u32, &'static str> = maplit::hashmap! {
        0 => "All Levels",
        1 => "Leather Treasure Map",
        2 => "Leather Treasure Map",
        3 => "Goatskin Treasure Map",
        4 => "Toadskin Treasure Map",
        5 => "Boarskin Treasure Map",
        6 => "Peisteskin Treasure Map",
        7 => "Leather Buried Treasure Map",
        8 => "Archaeoskin Treasure Map",
        9 => "Wyvernskin Treasure Map",
        10 => "Dragonskin Treasure Map",
        11 => "Gaganaskin Treasure Map",
        12 => "Gazelleskin Treasure Map",
        13 => "Seemingly Special Treasure Map",
        14 => "Gliderskin Treasure Map",
        15 => "Zonureskin Treasure Map",
        16 => "Presumably Special Treasure Map",
    };
}
