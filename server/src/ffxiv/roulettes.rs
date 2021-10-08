use std::collections::HashMap;

#[derive(Debug)]
pub struct RouletteInfo {
    pub name: &'static str,
    pub pvp: bool,
}

lazy_static::lazy_static! {
    pub static ref ROULETTES: HashMap<u32, RouletteInfo> = maplit::hashmap! {
        1 => RouletteInfo {
            name: "Duty Roulette: Leveling",
            pvp: false,
        },
        2 => RouletteInfo {
            name: "Duty Roulette: Level 50/60/70 Dungeons",
            pvp: false,
        },
        3 => RouletteInfo {
            name: "Duty Roulette: Main Scenario",
            pvp: false,
        },
        4 => RouletteInfo {
            name: "Duty Roulette: Guildhests",
            pvp: false,
        },
        5 => RouletteInfo {
            name: "Duty Roulette: Expert",
            pvp: false,
        },
        6 => RouletteInfo {
            name: "Duty Roulette: Trials",
            pvp: false,
        },
        7 => RouletteInfo {
            name: "Daily Challenge: Frontline",
            pvp: false,
        },
        8 => RouletteInfo {
            name: "Duty Roulette: Level 80 Dungeons",
            pvp: false,
        },
        9 => RouletteInfo {
            name: "Duty Roulette: Mentor",
            pvp: false,
        },
        11 => RouletteInfo {
            name: "The Feast (Training Match)",
            pvp: true,
        },
        13 => RouletteInfo {
            name: "The Feast (Ranked Match)",
            pvp: true,
        },
        15 => RouletteInfo {
            name: "Duty Roulette: Alliance Raids",
            pvp: false,
        },
        16 => RouletteInfo {
            name: "The Feast (Team Ranked Match)",
            pvp: true,
        },
        17 => RouletteInfo {
            name: "Duty Roulette: Normal Raids",
            pvp: false,
        },
        18 => RouletteInfo {
            name: "Chocobo Race: Sagolii Road",
            pvp: false,
        },
        19 => RouletteInfo {
            name: "Chocobo Race: Costa del Sol",
            pvp: false,
        },
        20 => RouletteInfo {
            name: "Chocobo Race: Tranquil Paths",
            pvp: false,
        },
        21 => RouletteInfo {
            name: "Chocobo Race: Random",
            pvp: false,
        },
        22 => RouletteInfo {
            name: "Chocobo Race: Sagolii Road (No Rewards)",
            pvp: false,
        },
        23 => RouletteInfo {
            name: "Chocobo Race: Costa del Sol (No Rewards)",
            pvp: false,
        },
        24 => RouletteInfo {
            name: "Chocobo Race: Tranquil Paths (No Rewards)",
            pvp: false,
        },
        25 => RouletteInfo {
            name: "Chocobo Race: Random (No Rewards)",
            pvp: false,
        },
        26 => RouletteInfo {
            name: "Chocobo Race: Random",
            pvp: false,
        },
        27 => RouletteInfo {
            name: "Chocobo Race: Random",
            pvp: false,
        },
        28 => RouletteInfo {
            name: "Chocobo Race: Random",
            pvp: false,
        },
        29 => RouletteInfo {
            name: "Chocobo Race: Random",
            pvp: false,
        },
        30 => RouletteInfo {
            name: "Chocobo Race: Random",
            pvp: false,
        },
        31 => RouletteInfo {
            name: "Chocobo Race: Random",
            pvp: false,
        },
        32 => RouletteInfo {
            name: "Chocobo Race: Random",
            pvp: false,
        },
        33 => RouletteInfo {
            name: "Chocobo Race: Random",
            pvp: false,
        },
        34 => RouletteInfo {
            name: "Chocobo Race: Random",
            pvp: false,
        },
        35 => RouletteInfo {
            name: "Chocobo Race: Random",
            pvp: false,
        },
        36 => RouletteInfo {
            name: "Chocobo Race: Random",
            pvp: false,
        },
        37 => RouletteInfo {
            name: "Chocobo Race: Random",
            pvp: false,
        },
        38 => RouletteInfo {
            name: "Chocobo Race: Random",
            pvp: false,
        },
    };
}
