use std::collections::HashMap;

#[derive(Debug)]
pub struct DutyInfo {
    pub name: &'static str,
    pub high_end: bool,
    pub content_kind: ContentKind,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum ContentKind {
    DutyRoulette = 1,
    Dungeons = 2,
    Guildhests = 3,
    Trials = 4,
    Raids = 5,
    PvP = 6,
    QuestBattles = 7,
    FATEs = 8,
    TreasureHunt = 9,
    Levequests = 10,
    GrandCompany = 11,
    Companions = 12,
    BeastTribeQuests = 13,
    OverallCompletion = 14,
    PlayerCommendation = 15,
    DisciplesoftheLand = 16,
    DisciplesoftheHand = 17,
    RetainerVentures = 18,
    GoldSaucer = 19,
    DeepDungeons = 21,
    WondrousTails = 24,
    CustomDeliveries = 25,
    Eureka = 26,
    UltimateRaids = 28,
    Other(u32),
}

impl ContentKind {
    fn from_u32(kind: u32) -> Self {
        match kind {
            1 => Self::DutyRoulette,
            2 => Self::Dungeons,
            3 => Self::Guildhests,
            4 => Self::Trials,
            5 => Self::Raids,
            6 => Self::PvP,
            7 => Self::QuestBattles,
            8 => Self::FATEs,
            9 => Self::TreasureHunt,
            10 => Self::Levequests,
            11 => Self::GrandCompany,
            12 => Self::Companions,
            13 => Self::BeastTribeQuests,
            14 => Self::OverallCompletion,
            15 => Self::PlayerCommendation,
            16 => Self::DisciplesoftheLand,
            17 => Self::DisciplesoftheHand,
            18 => Self::RetainerVentures,
            19 => Self::GoldSaucer,
            21 => Self::DeepDungeons,
            24 => Self::WondrousTails,
            25 => Self::CustomDeliveries,
            26 => Self::Eureka,
            28 => Self::UltimateRaids,
            x => Self::Other(x),
        }
    }
}

lazy_static::lazy_static! {
    pub static ref DUTIES: HashMap<u32, DutyInfo> = maplit::hashmap! {
        1 => DutyInfo {
            name: "The Thousand Maws of TotoRak",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        2 => DutyInfo {
            name: "The TamTara Deepcroft",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        3 => DutyInfo {
            name: "Copperbell Mines",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        4 => DutyInfo {
            name: "Sastasha",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        5 => DutyInfo {
            name: "The Aurum Vale",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        6 => DutyInfo {
            name: "Haukke Manor",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        7 => DutyInfo {
            name: "Halatali",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        8 => DutyInfo {
            name: "Brayflox's Longstop",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        9 => DutyInfo {
            name: "The Sunken Temple of Qarn",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        10 => DutyInfo {
            name: "The Wanderer's Palace",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        11 => DutyInfo {
            name: "The Stone Vigil",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        12 => DutyInfo {
            name: "Cutter's Cry",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        13 => DutyInfo {
            name: "Dzemael Darkhold",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        14 => DutyInfo {
            name: "Amdapor Keep",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        15 => DutyInfo {
            name: "Castrum Meridianum",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        16 => DutyInfo {
            name: "The Praetorium",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        17 => DutyInfo {
            name: "Pharos Sirius",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        18 => DutyInfo {
            name: "Copperbell Mines (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        19 => DutyInfo {
            name: "Haukke Manor (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        20 => DutyInfo {
            name: "Brayflox's Longstop (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        21 => DutyInfo {
            name: "Halatali (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        22 => DutyInfo {
            name: "The Lost City of Amdapor",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        23 => DutyInfo {
            name: "Hullbreaker Isle",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        24 => DutyInfo {
            name: "The TamTara Deepcroft (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        25 => DutyInfo {
            name: "The Stone Vigil (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        26 => DutyInfo {
            name: "The Sunken Temple of Qarn (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        27 => DutyInfo {
            name: "Snowcloak",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        28 => DutyInfo {
            name: "Sastasha (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        29 => DutyInfo {
            name: "Amdapor Keep (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        30 => DutyInfo {
            name: "The Wanderer's Palace (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        31 => DutyInfo {
            name: "The Great Gubal Library",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        32 => DutyInfo {
            name: "The Keeper of the Lake",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        33 => DutyInfo {
            name: "Neverreap",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        34 => DutyInfo {
            name: "The Vault",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        35 => DutyInfo {
            name: "The Fractal Continuum",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        36 => DutyInfo {
            name: "The Dusk Vigil",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        37 => DutyInfo {
            name: "Sohm Al",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        38 => DutyInfo {
            name: "The Aetherochemical Research Facility",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        39 => DutyInfo {
            name: "The Aery",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        40 => DutyInfo {
            name: "Pharos Sirius (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        41 => DutyInfo {
            name: "Saint Mocianne's Arboretum",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        42 => DutyInfo {
            name: "Basic Training: Enemy Parties",
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        43 => DutyInfo {
            name: "Under the Armor",
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        44 => DutyInfo {
            name: "Basic Training: Enemy Strongholds",
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        45 => DutyInfo {
            name: "Hero on the Half Shell",
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        46 => DutyInfo {
            name: "Pulling Poison Posies",
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        47 => DutyInfo {
            name: "Stinging Back",
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        48 => DutyInfo {
            name: "All's Well that Ends in the Well",
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        49 => DutyInfo {
            name: "Flicking Sticks and Taking Names",
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        50 => DutyInfo {
            name: "More than a Feeler",
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        51 => DutyInfo {
            name: "Annoy the Void",
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        52 => DutyInfo {
            name: "Shadow and Claw",
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        53 => DutyInfo {
            name: "Long Live the Queen",
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        54 => DutyInfo {
            name: "Ward Up",
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        55 => DutyInfo {
            name: "Solemn Trinity",
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        56 => DutyInfo {
            name: "The Bowl of Embers",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        57 => DutyInfo {
            name: "The Navel",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        58 => DutyInfo {
            name: "The Howling Eye",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        59 => DutyInfo {
            name: "The Bowl of Embers (Hard)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        60 => DutyInfo {
            name: "The Navel (Hard)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        61 => DutyInfo {
            name: "The Howling Eye (Hard)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        62 => DutyInfo {
            name: "Cape Westwind",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        63 => DutyInfo {
            name: "The Bowl of Embers (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        64 => DutyInfo {
            name: "The Navel (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        65 => DutyInfo {
            name: "The Howling Eye (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        66 => DutyInfo {
            name: "Thornmarch (Hard)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        67 => DutyInfo {
            name: "Thornmarch (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        68 => DutyInfo {
            name: "The Minstrel's Ballad: Ultima's Bane",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        69 => DutyInfo {
            name: "Special Event III",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        70 => DutyInfo {
            name: "Special Event I",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        71 => DutyInfo {
            name: "Special Event II",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        72 => DutyInfo {
            name: "The Whorleater (Hard)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        73 => DutyInfo {
            name: "The Whorleater (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        74 => DutyInfo {
            name: "A Relic Reborn: the Chimera",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        75 => DutyInfo {
            name: "A Relic Reborn: the Hydra",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        76 => DutyInfo {
            name: "Battle on the Big Bridge",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        77 => DutyInfo {
            name: "The Striking Tree (Hard)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        78 => DutyInfo {
            name: "The Striking Tree (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        79 => DutyInfo {
            name: "The Akh Afah Amphitheatre (Hard)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        80 => DutyInfo {
            name: "The Akh Afah Amphitheatre (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        81 => DutyInfo {
            name: "The Dragon's Neck",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        82 => DutyInfo {
            name: "Urth's Fount",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        83 => DutyInfo {
            name: "The Steps of Faith",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        84 => DutyInfo {
            name: "The Chrysalis",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        85 => DutyInfo {
            name: "Battle in the Big Keep",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        86 => DutyInfo {
            name: "Thok ast Thok (Hard)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        87 => DutyInfo {
            name: "Thok ast Thok (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        88 => DutyInfo {
            name: "The Limitless Blue (Hard)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        89 => DutyInfo {
            name: "The Limitless Blue (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        90 => DutyInfo {
            name: "The Singularity Reactor",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        91 => DutyInfo {
            name: "The Minstrel's Ballad: Thordan's Reign",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        92 => DutyInfo {
            name: "The Labyrinth of the Ancients",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        93 => DutyInfo {
            name: "The Binding Coil of Bahamut - Turn 1",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        94 => DutyInfo {
            name: "The Binding Coil of Bahamut - Turn 2",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        95 => DutyInfo {
            name: "The Binding Coil of Bahamut - Turn 3",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        96 => DutyInfo {
            name: "The Binding Coil of Bahamut - Turn 4",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        97 => DutyInfo {
            name: "The Binding Coil of Bahamut - Turn 5",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        98 => DutyInfo {
            name: "The Second Coil of Bahamut - Turn 1",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        99 => DutyInfo {
            name: "The Second Coil of Bahamut - Turn 2",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        100 => DutyInfo {
            name: "The Second Coil of Bahamut - Turn 3",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        101 => DutyInfo {
            name: "The Second Coil of Bahamut - Turn 4",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        102 => DutyInfo {
            name: "Syrcus Tower",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        103 => DutyInfo {
            name: "The Second Coil of Bahamut (Savage) - Turn 1",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        104 => DutyInfo {
            name: "The Second Coil of Bahamut (Savage) - Turn 2",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        105 => DutyInfo {
            name: "The Second Coil of Bahamut (Savage) - Turn 3",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        106 => DutyInfo {
            name: "The Second Coil of Bahamut (Savage) - Turn 4",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        107 => DutyInfo {
            name: "The Final Coil of Bahamut - Turn 1",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        108 => DutyInfo {
            name: "The Final Coil of Bahamut - Turn 2",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        109 => DutyInfo {
            name: "The Final Coil of Bahamut - Turn 3",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        110 => DutyInfo {
            name: "The Final Coil of Bahamut - Turn 4",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        111 => DutyInfo {
            name: "The World of Darkness",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        112 => DutyInfo {
            name: "Alexander - The Fist of the Father",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        113 => DutyInfo {
            name: "Alexander - The Cuff of the Father",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        114 => DutyInfo {
            name: "Alexander - The Arm of the Father",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        115 => DutyInfo {
            name: "Alexander - The Burden of the Father",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        116 => DutyInfo {
            name: "Alexander - The Fist of the Father (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        117 => DutyInfo {
            name: "Alexander - The Cuff of the Father (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        118 => DutyInfo {
            name: "Alexander - The Arm of the Father (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        119 => DutyInfo {
            name: "Alexander - The Burden of the Father (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        120 => DutyInfo {
            name: "The Void Ark",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        127 => DutyInfo {
            name: "The Borderland Ruins (Secure)",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        130 => DutyInfo {
            name: "Seal Rock (Seize)",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        131 => DutyInfo {
            name: "The Diadem (Easy)",
            high_end: false,
            content_kind: ContentKind::Other(23),
        },
        132 => DutyInfo {
            name: "The Diadem",
            high_end: false,
            content_kind: ContentKind::Other(23),
        },
        133 => DutyInfo {
            name: "The Diadem (Hard)",
            high_end: false,
            content_kind: ContentKind::Other(23),
        },
        134 => DutyInfo {
            name: "Containment Bay S1T7",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        135 => DutyInfo {
            name: "Containment Bay S1T7 (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        136 => DutyInfo {
            name: "Alexander - The Fist of the Son",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        137 => DutyInfo {
            name: "Alexander - The Cuff of the Son",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        138 => DutyInfo {
            name: "Alexander - The Arm of the Son",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        139 => DutyInfo {
            name: "Alexander - The Burden of the Son",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        140 => DutyInfo {
            name: "The Lost City of Amdapor (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        141 => DutyInfo {
            name: "The Antitower",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        143 => DutyInfo {
            name: "The Feast (4 on 4 - Training)",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        145 => DutyInfo {
            name: "The Feast (4 on 4 - Ranked)",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        147 => DutyInfo {
            name: "Alexander - The Fist of the Son (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        148 => DutyInfo {
            name: "Alexander - The Cuff of the Son (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        149 => DutyInfo {
            name: "Alexander - The Arm of the Son (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        150 => DutyInfo {
            name: "Alexander - The Burden of the Son (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        151 => DutyInfo {
            name: "Avoid Area of Effect Attacks",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        152 => DutyInfo {
            name: "Execute a Combo to Increase Enmity",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        153 => DutyInfo {
            name: "Execute a Combo in Battle",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        154 => DutyInfo {
            name: "Accrue Enmity from Multiple Targets",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        155 => DutyInfo {
            name: "Engage Multiple Targets",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        156 => DutyInfo {
            name: "Execute a Ranged Attack to Increase Enmity",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        157 => DutyInfo {
            name: "Engage Enemy Reinforcements",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        158 => DutyInfo {
            name: "Assist Allies in Defeating a Target",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        159 => DutyInfo {
            name: "Defeat an Occupied Target",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        160 => DutyInfo {
            name: "Avoid Engaged Targets",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        161 => DutyInfo {
            name: "Engage Enemy Reinforcements",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        162 => DutyInfo {
            name: "Interact with the Battlefield",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        163 => DutyInfo {
            name: "Heal an Ally",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        164 => DutyInfo {
            name: "Heal Multiple Allies",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        165 => DutyInfo {
            name: "Avoid Engaged Targets",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        166 => DutyInfo {
            name: "Final Exercise",
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        167 => DutyInfo {
            name: "A Spectacle for the Ages",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        168 => DutyInfo {
            name: "The Weeping City of Mhach",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        169 => DutyInfo {
            name: "The Final Steps of Faith",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        170 => DutyInfo {
            name: "The Minstrel's Ballad: Nidhogg's Rage",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        171 => DutyInfo {
            name: "Sohr Khai",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        172 => DutyInfo {
            name: "Hullbreaker Isle (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        173 => DutyInfo {
            name: "A Bloody Reunion",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        174 => DutyInfo {
            name: "The Palace of the Dead (Floors 1-10)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        175 => DutyInfo {
            name: "The Palace of the Dead (Floors 11-20)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        176 => DutyInfo {
            name: "The Palace of the Dead (Floors 21-30)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        177 => DutyInfo {
            name: "The Palace of the Dead (Floors 31-40)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        178 => DutyInfo {
            name: "The Palace of the Dead (Floors 41-50)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        179 => DutyInfo {
            name: "The Aquapolis",
            high_end: false,
            content_kind: ContentKind::TreasureHunt,
        },
        180 => DutyInfo {
            name: "The Fields of Glory (Shatter)",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        181 => DutyInfo {
            name: "The Haunted Manor",
            high_end: false,
            content_kind: ContentKind::Other(22),
        },
        182 => DutyInfo {
            name: "Xelphatol",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        183 => DutyInfo {
            name: "Containment Bay P1T6",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        184 => DutyInfo {
            name: "Containment Bay P1T6 (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        186 => DutyInfo {
            name: "Alexander - The Eyes of the Creator",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        187 => DutyInfo {
            name: "Alexander - The Breath of the Creator",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        188 => DutyInfo {
            name: "Alexander - The Heart of the Creator",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        189 => DutyInfo {
            name: "Alexander - The Soul of the Creator",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        190 => DutyInfo {
            name: "Alexander - The Eyes of the Creator (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        191 => DutyInfo {
            name: "Alexander - The Breath of the Creator (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        192 => DutyInfo {
            name: "Alexander - The Heart of the Creator (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        193 => DutyInfo {
            name: "Alexander - The Soul of the Creator (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        194 => DutyInfo {
            name: "One Life for One World",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        195 => DutyInfo {
            name: "The Triple Triad Battlehall",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        196 => DutyInfo {
            name: "The Great Gubal Library (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        197 => DutyInfo {
            name: "LoVM: Player Battle (RP)",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        198 => DutyInfo {
            name: "LoVM: Tournament",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        199 => DutyInfo {
            name: "LoVM: Player Battle (Non-RP)",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        201 => DutyInfo {
            name: "The Feast (Custom Match - Feasting Grounds)",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        202 => DutyInfo {
            name: "The Diadem Hunting Grounds (Easy)",
            high_end: false,
            content_kind: ContentKind::Other(23),
        },
        203 => DutyInfo {
            name: "The Diadem Hunting Grounds",
            high_end: false,
            content_kind: ContentKind::Other(23),
        },
        204 => DutyInfo {
            name: "The Palace of the Dead (Floors 51-60)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        205 => DutyInfo {
            name: "The Palace of the Dead (Floors 61-70)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        206 => DutyInfo {
            name: "The Palace of the Dead (Floors 71-80)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        207 => DutyInfo {
            name: "The Palace of the Dead (Floors 81-90)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        208 => DutyInfo {
            name: "The Palace of the Dead (Floors 91-100)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        209 => DutyInfo {
            name: "The Palace of the Dead (Floors 101-110)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        210 => DutyInfo {
            name: "The Palace of the Dead (Floors 111-120)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        211 => DutyInfo {
            name: "The Palace of the Dead (Floors 121-130)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        212 => DutyInfo {
            name: "The Palace of the Dead (Floors 131-140)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        213 => DutyInfo {
            name: "The Palace of the Dead (Floors 141-150)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        214 => DutyInfo {
            name: "The Palace of the Dead (Floors 151-160)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        215 => DutyInfo {
            name: "The Palace of the Dead (Floors 161-170)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        216 => DutyInfo {
            name: "The Palace of the Dead (Floors 171-180)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        217 => DutyInfo {
            name: "The Palace of the Dead (Floors 181-190)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        218 => DutyInfo {
            name: "The Palace of the Dead (Floors 191-200)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        219 => DutyInfo {
            name: "Baelsar's Wall",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        220 => DutyInfo {
            name: "Dun Scaith",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        221 => DutyInfo {
            name: "Sohm Al (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        222 => DutyInfo {
            name: "The Carteneau Flats: Heliodrome",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        223 => DutyInfo {
            name: "Containment Bay Z1T9",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        224 => DutyInfo {
            name: "Containment Bay Z1T9 (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        225 => DutyInfo {
            name: "The Diadem - Trials of the Fury",
            high_end: false,
            content_kind: ContentKind::Other(23),
        },
        228 => DutyInfo {
            name: "The Feast (4 on 4 - Training)",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        230 => DutyInfo {
            name: "The Feast (4 on 4 - Ranked)",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        233 => DutyInfo {
            name: "The Feast (Custom Match - Lichenweed)",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        234 => DutyInfo {
            name: "The Diadem - Trials of the Matron",
            high_end: false,
            content_kind: ContentKind::Other(23),
        },
        235 => DutyInfo {
            name: "Shisui of the Violet Tides",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        236 => DutyInfo {
            name: "The Temple of the Fist",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        237 => DutyInfo {
            name: "It's Probably a Trap",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        238 => DutyInfo {
            name: "The Sirensong Sea",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        239 => DutyInfo {
            name: "The Royal Menagerie",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        240 => DutyInfo {
            name: "Bardam's Mettle",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        241 => DutyInfo {
            name: "Doma Castle",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        242 => DutyInfo {
            name: "Castrum Abania",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        243 => DutyInfo {
            name: "The Pool of Tribute",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        244 => DutyInfo {
            name: "The Pool of Tribute (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        245 => DutyInfo {
            name: "With Heart and Steel",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        246 => DutyInfo {
            name: "Naadam",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        247 => DutyInfo {
            name: "Ala Mhigo",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        248 => DutyInfo {
            name: "Blood on the Deck",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        249 => DutyInfo {
            name: "The Face of True Evil",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        250 => DutyInfo {
            name: "Matsuba Mayhem",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        251 => DutyInfo {
            name: "The Battle on Bekko",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        252 => DutyInfo {
            name: "Deltascape V1.0",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        253 => DutyInfo {
            name: "Deltascape V2.0",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        254 => DutyInfo {
            name: "Deltascape V3.0",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        255 => DutyInfo {
            name: "Deltascape V4.0",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        256 => DutyInfo {
            name: "Deltascape V1.0 (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        257 => DutyInfo {
            name: "Deltascape V2.0 (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        258 => DutyInfo {
            name: "Deltascape V3.0 (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        259 => DutyInfo {
            name: "Deltascape V4.0 (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        260 => DutyInfo {
            name: "Curious Gorge Meets His Match",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        261 => DutyInfo {
            name: "In Thal's Name",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        262 => DutyInfo {
            name: "Kugane Castle",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        263 => DutyInfo {
            name: "Emanation",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        264 => DutyInfo {
            name: "Emanation (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        265 => DutyInfo {
            name: "Our Unsung Heroes",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        266 => DutyInfo {
            name: "The Heart of the Problem",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        267 => DutyInfo {
            name: "Dark as the Night Sky",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        268 => DutyInfo {
            name: "The Lost Canals of Uznair",
            high_end: false,
            content_kind: ContentKind::TreasureHunt,
        },
        269 => DutyInfo {
            name: "The Resonant",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        270 => DutyInfo {
            name: "Raising the Sword",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        271 => DutyInfo {
            name: "The Orphans and the Broken Blade",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        272 => DutyInfo {
            name: "Our Compromise",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        273 => DutyInfo {
            name: "Dragon Sound",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        274 => DutyInfo {
            name: "When Clans Collide",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        275 => DutyInfo {
            name: "Interdimensional Rift",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        276 => DutyInfo {
            name: "The Hidden Canals of Uznair",
            high_end: false,
            content_kind: ContentKind::TreasureHunt,
        },
        277 => DutyInfo {
            name: "Astragalos",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        278 => DutyInfo {
            name: "The Minstrel's Ballad: Shinryu's Domain",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        279 => DutyInfo {
            name: "The Drowned City of Skalla",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        280 => DutyInfo {
            name: "The Unending Coil of Bahamut (Ultimate)",
            high_end: true,
            content_kind: ContentKind::UltimateRaids,
        },
        281 => DutyInfo {
            name: "The Royal City of Rabanastre",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        282 => DutyInfo {
            name: "Return of the Bull",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        283 => DutyInfo {
            name: "The Forbidden Land, Eureka Anemos",
            high_end: false,
            content_kind: ContentKind::Eureka,
        },
        284 => DutyInfo {
            name: "Hells' Lid",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        285 => DutyInfo {
            name: "The Fractal Continuum (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        286 => DutyInfo {
            name: "Sigmascape V1.0",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        287 => DutyInfo {
            name: "Sigmascape V2.0",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        288 => DutyInfo {
            name: "Sigmascape V3.0",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        289 => DutyInfo {
            name: "Sigmascape V4.0",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        290 => DutyInfo {
            name: "The Jade Stoa",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        291 => DutyInfo {
            name: "The Jade Stoa (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        292 => DutyInfo {
            name: "Sigmascape V1.0 (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        293 => DutyInfo {
            name: "Sigmascape V2.0 (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        294 => DutyInfo {
            name: "Sigmascape V3.0 (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        295 => DutyInfo {
            name: "Sigmascape V4.0 (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        473 => DutyInfo {
            name: "The Valentione's Ceremony",
            high_end: false,
            content_kind: ContentKind::Other(22),
        },
        474 => DutyInfo {
            name: "The Great Hunt",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        475 => DutyInfo {
            name: "The Great Hunt (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        476 => DutyInfo {
            name: "The Feast (Team Ranked)",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        478 => DutyInfo {
            name: "The Feast (Ranked)",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        479 => DutyInfo {
            name: "The Feast (Training)",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        480 => DutyInfo {
            name: "The Feast (Custom Match - Crystal Tower)",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        481 => DutyInfo {
            name: "Chocobo Race: Tutorial",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        482 => DutyInfo {
            name: "Race 1 - Hugging the Inside",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        483 => DutyInfo {
            name: "Race 2 - Keep Away",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        484 => DutyInfo {
            name: "Race 3 - Inability",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        485 => DutyInfo {
            name: "Race 4 - Heavy Hooves",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        486 => DutyInfo {
            name: "Race 5 - Defending the Rush",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        487 => DutyInfo {
            name: "Race 6 - Road Rivals",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        488 => DutyInfo {
            name: "Race 7 - Field of Dreams",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        489 => DutyInfo {
            name: "Race 8 - Playing Both Ends",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        490 => DutyInfo {
            name: "Race 9 - Stamina",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        491 => DutyInfo {
            name: "Race 10 - Cat and Mouse",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        492 => DutyInfo {
            name: "Race 11 - Mad Dash",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        493 => DutyInfo {
            name: "Race 12 - Bag of Tricks",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        494 => DutyInfo {
            name: "Race 13 - Tag Team",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        495 => DutyInfo {
            name: "Race 14 - Heavier Hooves",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        496 => DutyInfo {
            name: "Race 15 - Ultimatum",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        497 => DutyInfo {
            name: "Chocobo Race: Sagolii Road",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        498 => DutyInfo {
            name: "Chocobo Race: Costa del Sol",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        499 => DutyInfo {
            name: "Chocobo Race: Tranquil Paths",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        500 => DutyInfo {
            name: "Chocobo Race: Sagolii Road",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        501 => DutyInfo {
            name: "Chocobo Race: Costa del Sol",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        502 => DutyInfo {
            name: "Chocobo Race: Tranquil Paths",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        503 => DutyInfo {
            name: "Chocobo Race: Sagolii Road",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        504 => DutyInfo {
            name: "Chocobo Race: Costa del Sol",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        505 => DutyInfo {
            name: "Chocobo Race: Tranquil Paths",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        506 => DutyInfo {
            name: "Chocobo Race: Sagolii Road",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        507 => DutyInfo {
            name: "Chocobo Race: Costa del Sol",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        508 => DutyInfo {
            name: "Chocobo Race: Tranquil Paths",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        509 => DutyInfo {
            name: "Chocobo Race: Sagolii Road",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        510 => DutyInfo {
            name: "Chocobo Race: Costa del Sol",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        511 => DutyInfo {
            name: "Chocobo Race: Tranquil Paths",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        512 => DutyInfo {
            name: "Chocobo Race: Sagolii Road",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        513 => DutyInfo {
            name: "Chocobo Race: Costa del Sol",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        514 => DutyInfo {
            name: "Chocobo Race: Tranquil Paths",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        515 => DutyInfo {
            name: "Chocobo Race: Sagolii Road",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        516 => DutyInfo {
            name: "Chocobo Race: Costa del Sol",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        517 => DutyInfo {
            name: "Chocobo Race: Tranquil Paths",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        518 => DutyInfo {
            name: "Chocobo Race: Sagolii Road",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        519 => DutyInfo {
            name: "Chocobo Race: Costa del Sol",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        520 => DutyInfo {
            name: "Chocobo Race: Tranquil Paths",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        521 => DutyInfo {
            name: "Chocobo Race: Sagolii Road",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        522 => DutyInfo {
            name: "Chocobo Race: Costa del Sol",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        523 => DutyInfo {
            name: "Chocobo Race: Tranquil Paths",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        524 => DutyInfo {
            name: "Chocobo Race: Sagolii Road",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        525 => DutyInfo {
            name: "Chocobo Race: Costa del Sol",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        526 => DutyInfo {
            name: "Chocobo Race: Tranquil Paths",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        527 => DutyInfo {
            name: "Chocobo Race: Sagolii Road",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        528 => DutyInfo {
            name: "Chocobo Race: Costa del Sol",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        529 => DutyInfo {
            name: "Chocobo Race: Tranquil Paths",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        530 => DutyInfo {
            name: "Chocobo Race: Sagolii Road",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        531 => DutyInfo {
            name: "Chocobo Race: Costa del Sol",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        532 => DutyInfo {
            name: "Chocobo Race: Tranquil Paths",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        533 => DutyInfo {
            name: "Chocobo Race: Sagolii Road",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        534 => DutyInfo {
            name: "Chocobo Race: Costa del Sol",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        535 => DutyInfo {
            name: "Chocobo Race: Tranquil Paths",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        536 => DutyInfo {
            name: "The Swallow's Compass",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        537 => DutyInfo {
            name: "Castrum Fluminis",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        538 => DutyInfo {
            name: "The Minstrel's Ballad: Tsukuyomi's Pain",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        539 => DutyInfo {
            name: "The Weapon's Refrain (Ultimate)",
            high_end: true,
            content_kind: ContentKind::UltimateRaids,
        },
        540 => DutyInfo {
            name: "Heaven-on-High  (Floors 1-10)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        541 => DutyInfo {
            name: "Heaven-on-High  (Floors 11-20)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        542 => DutyInfo {
            name: "Heaven-on-High  (Floors 21-30)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        543 => DutyInfo {
            name: "Heaven-on-High  (Floors 31-40)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        544 => DutyInfo {
            name: "Heaven-on-High  (Floors 41-50)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        545 => DutyInfo {
            name: "Heaven-on-High  (Floors 51-60)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        546 => DutyInfo {
            name: "Heaven-on-High  (Floors 61-70)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        547 => DutyInfo {
            name: "Heaven-on-High  (Floors 71-80)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        548 => DutyInfo {
            name: "Heaven-on-High  (Floors 81-90)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        549 => DutyInfo {
            name: "Heaven-on-High  (Floors 91-100)",
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        550 => DutyInfo {
            name: "The Ridorana Lighthouse",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        552 => DutyInfo {
            name: "Stage 1: Tutorial",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        553 => DutyInfo {
            name: "Stage 2: Hatching a Plan",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        554 => DutyInfo {
            name: "Stage 3: The First Move",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        555 => DutyInfo {
            name: "Stage 4: Little Big Beast",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        556 => DutyInfo {
            name: "Stage 5: Turning Tribes",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        557 => DutyInfo {
            name: "Stage 6: Off the Deepcroft",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        558 => DutyInfo {
            name: "Stage 7: Rivals",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        559 => DutyInfo {
            name: "Stage 8: Always Darkest",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        560 => DutyInfo {
            name: "Stage 9: Mine Your Minions",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        561 => DutyInfo {
            name: "Stage 10: Children of Mandragora",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        562 => DutyInfo {
            name: "Stage 11: The Queen and I",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        563 => DutyInfo {
            name: "Stage 12: Breakout",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        564 => DutyInfo {
            name: "Stage 13: My Name Is Cid",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        565 => DutyInfo {
            name: "Stage 14: Like a Nut",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        566 => DutyInfo {
            name: "Stage 15: Urth's Spout",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        567 => DutyInfo {
            name: "Stage 16: Exodus",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        568 => DutyInfo {
            name: "Stage 17: Over the Wall",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        569 => DutyInfo {
            name: "Stage 18: The Hunt",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        570 => DutyInfo {
            name: "Stage 19: Battle on the Bitty Bridge",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        571 => DutyInfo {
            name: "Stage 20: Guiding Light",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        572 => DutyInfo {
            name: "Stage 21: Wise Words",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        573 => DutyInfo {
            name: "Stage 22: World of Poor Lighting",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        574 => DutyInfo {
            name: "Stage 23: The Binding Coil",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        575 => DutyInfo {
            name: "Stage 24: The Final Coil",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        576 => DutyInfo {
            name: "LoVM: Master Battle",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        577 => DutyInfo {
            name: "LoVM: Master Battle (Hard)",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        578 => DutyInfo {
            name: "LoVM: Master Battle (Extreme)",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        579 => DutyInfo {
            name: "LoVM: Master Tournament",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        580 => DutyInfo {
            name: "The Feast (Team Custom Match - Crystal Tower)",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        581 => DutyInfo {
            name: "The Forbidden Land, Eureka Pagos",
            high_end: false,
            content_kind: ContentKind::Eureka,
        },
        582 => DutyInfo {
            name: "Emissary of the Dawn",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        583 => DutyInfo {
            name: "The Calamity Retold",
            high_end: false,
            content_kind: ContentKind::Other(22),
        },
        584 => DutyInfo {
            name: "Saint Mocianne's Arboretum (Hard)",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        585 => DutyInfo {
            name: "The Burn",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        586 => DutyInfo {
            name: "The Shifting Altars of Uznair",
            high_end: false,
            content_kind: ContentKind::TreasureHunt,
        },
        587 => DutyInfo {
            name: "Alphascape V1.0",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        588 => DutyInfo {
            name: "Alphascape V2.0",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        589 => DutyInfo {
            name: "Alphascape V3.0",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        590 => DutyInfo {
            name: "Alphascape V4.0",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        591 => DutyInfo {
            name: "Alphascape V1.0 (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        592 => DutyInfo {
            name: "Alphascape V2.0 (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        593 => DutyInfo {
            name: "Alphascape V3.0 (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        594 => DutyInfo {
            name: "Alphascape V4.0 (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        595 => DutyInfo {
            name: "Kugane Ohashi",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        596 => DutyInfo {
            name: "Hells' Kier",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        597 => DutyInfo {
            name: "Hells' Kier (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        598 => DutyInfo {
            name: "The Forbidden Land, Eureka Pyros",
            high_end: false,
            content_kind: ContentKind::Eureka,
        },
        599 => DutyInfo {
            name: "Hidden Gorge",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        600 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        601 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        602 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        603 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        604 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        605 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        606 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        607 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        608 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        609 => DutyInfo {
            name: "The Will of the Moon",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        610 => DutyInfo {
            name: "All's Well That Starts Well",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        611 => DutyInfo {
            name: "The Ghimlyt Dark",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        612 => DutyInfo {
            name: "Much Ado About Pudding",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        613 => DutyInfo {
            name: "Waiting for Golem",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        614 => DutyInfo {
            name: "Gentlemen Prefer Swords",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        615 => DutyInfo {
            name: "The Threepenny Turtles",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        616 => DutyInfo {
            name: "Eye Society",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        617 => DutyInfo {
            name: "A Chorus Slime",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        618 => DutyInfo {
            name: "Bomb-edy of Errors",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        619 => DutyInfo {
            name: "To Kill a Mockingslime",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        620 => DutyInfo {
            name: "A Little Knight Music",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        621 => DutyInfo {
            name: "Some Like It Excruciatingly Hot",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        622 => DutyInfo {
            name: "The Plant-om of the Opera",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        623 => DutyInfo {
            name: "Beauty and a Beast",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        624 => DutyInfo {
            name: "Blobs in the Woods",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        625 => DutyInfo {
            name: "The Me Nobody Nodes",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        626 => DutyInfo {
            name: "Sunset Bull-evard",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        627 => DutyInfo {
            name: "The Sword of Music",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        628 => DutyInfo {
            name: "Midsummer Night's Explosion",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        629 => DutyInfo {
            name: "On a Clear Day You Can Smell Forever",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        630 => DutyInfo {
            name: "Miss Typhon",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        631 => DutyInfo {
            name: "Chimera on a Hot Tin Roof",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        632 => DutyInfo {
            name: "Here Comes the Boom",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        633 => DutyInfo {
            name: "Behemoths and Broomsticks",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        634 => DutyInfo {
            name: "Amazing Technicolor Pit Fiends",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        635 => DutyInfo {
            name: "Dirty Rotten Azulmagia",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        636 => DutyInfo {
            name: "The Orbonne Monastery",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        637 => DutyInfo {
            name: "The Wreath of Snakes",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        638 => DutyInfo {
            name: "The Wreath of Snakes (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        639 => DutyInfo {
            name: "The Forbidden Land, Eureka Hydatos",
            high_end: false,
            content_kind: ContentKind::Eureka,
        },
        640 => DutyInfo {
            name: "Air Force One",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        641 => DutyInfo {
            name: "Air Force One",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        642 => DutyInfo {
            name: "Air Force One",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        643 => DutyInfo {
            name: "Novice Mahjong (Full Ranked Match)",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        644 => DutyInfo {
            name: "Advanced Mahjong (Full Ranked Match)",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        645 => DutyInfo {
            name: "Four-player Mahjong (Full Match, Kuitan Enabled)",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        646 => DutyInfo {
            name: "Messenger of the Winds",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        648 => DutyInfo {
            name: "A Requiem for Heroes",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        649 => DutyInfo {
            name: "Dohn Mheg",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        650 => DutyInfo {
            name: "Four-player Mahjong (Full Match, Kuitan Disabled)",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        651 => DutyInfo {
            name: "The Qitana Ravel",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        652 => DutyInfo {
            name: "Amaurot",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        653 => DutyInfo {
            name: "Eden's Gate: Resurrection",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        654 => DutyInfo {
            name: "Eden's Gate: Resurrection (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        655 => DutyInfo {
            name: "The Twinning",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        656 => DutyInfo {
            name: "Malikah's Well",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        657 => DutyInfo {
            name: "The Dancing Plague",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        658 => DutyInfo {
            name: "The Dancing Plague (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        659 => DutyInfo {
            name: "Mt. Gulg",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        661 => DutyInfo {
            name: "Akadaemia Anyder",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        666 => DutyInfo {
            name: "The Crown of the Immaculate",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        667 => DutyInfo {
            name: "The Crown of the Immaculate (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        676 => DutyInfo {
            name: "Holminster Switch",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        678 => DutyInfo {
            name: "The Hardened Heart",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        679 => DutyInfo {
            name: "The Lost and the Found",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        680 => DutyInfo {
            name: "Coming Clean",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        681 => DutyInfo {
            name: "Legend of the Not-so-hidden Temple",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        682 => DutyInfo {
            name: "Eden's Gate: Inundation",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        683 => DutyInfo {
            name: "Eden's Gate: Inundation (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        684 => DutyInfo {
            name: "Eden's Gate: Descent",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        685 => DutyInfo {
            name: "Eden's Gate: Descent (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        686 => DutyInfo {
            name: "Nyelbert's Lament",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        687 => DutyInfo {
            name: "The Dying Gasp",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        688 => DutyInfo {
            name: "The Dungeons of Lyhe Ghiah",
            high_end: false,
            content_kind: ContentKind::TreasureHunt,
        },
        689 => DutyInfo {
            name: "Eden's Gate: Sepulture",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        690 => DutyInfo {
            name: "Eden's Gate: Sepulture (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        691 => DutyInfo {
            name: "The Hunter's Legacy",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        692 => DutyInfo {
            name: "The Grand Cosmos",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        693 => DutyInfo {
            name: "The Minstrel's Ballad: Hades's Elegy",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        694 => DutyInfo {
            name: "The Epic of Alexander (Ultimate)",
            high_end: true,
            content_kind: ContentKind::UltimateRaids,
        },
        695 => DutyInfo {
            name: "Papa Mia",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        696 => DutyInfo {
            name: "Lock up Your Snorters",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        697 => DutyInfo {
            name: "Dangerous When Dead",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        698 => DutyInfo {
            name: "Red, Fraught, and Blue",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        699 => DutyInfo {
            name: "The Catch of the Siegfried",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        700 => DutyInfo {
            name: "The Copied Factory",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        701 => DutyInfo {
            name: "Onsal Hakair (Danshig Naadam)",
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        702 => DutyInfo {
            name: "Vows of Virtue, Deeds of Cruelty",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        703 => DutyInfo {
            name: "As the Heart Bids",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        705 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        706 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        707 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        708 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        709 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        710 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        711 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        712 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        713 => DutyInfo {
            name: "Leap of Faith",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        714 => DutyInfo {
            name: "Anamnesis Anyder",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        715 => DutyInfo {
            name: "Eden's Verse: Fulmination",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        716 => DutyInfo {
            name: "Eden's Verse: Fulmination (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        717 => DutyInfo {
            name: "Cinder Drift",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        718 => DutyInfo {
            name: "Cinder Drift (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        719 => DutyInfo {
            name: "Eden's Verse: Furor",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        720 => DutyInfo {
            name: "Eden's Verse: Furor (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        721 => DutyInfo {
            name: "Ocean Fishing",
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        722 => DutyInfo {
            name: "The Diadem",
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        723 => DutyInfo {
            name: "The Bozja Incident",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        724 => DutyInfo {
            name: "A Sleep Disturbed",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        725 => DutyInfo {
            name: "Memoria Misera (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        726 => DutyInfo {
            name: "Eden's Verse: Iconoclasm",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        727 => DutyInfo {
            name: "Eden's Verse: Iconoclasm (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        728 => DutyInfo {
            name: "Eden's Verse: Refulgence",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        729 => DutyInfo {
            name: "Eden's Verse: Refulgence (Savage)",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        730 => DutyInfo {
            name: "Ocean Fishing",
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        731 => DutyInfo {
            name: "Ocean Fishing",
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        732 => DutyInfo {
            name: "Ocean Fishing",
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        733 => DutyInfo {
            name: "Ocean Fishing",
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        734 => DutyInfo {
            name: "Ocean Fishing",
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        735 => DutyInfo {
            name: "The Bozjan Southern Front",
            high_end: false,
            content_kind: ContentKind::Other(29),
        },
        736 => DutyInfo {
            name: "The Puppets' Bunker",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        737 => DutyInfo {
            name: "The Heroes' Gauntlet",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        738 => DutyInfo {
            name: "The Seat of Sacrifice",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        739 => DutyInfo {
            name: "The Seat of Sacrifice (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        740 => DutyInfo {
            name: "Sleep Now in Sapphire",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        741 => DutyInfo {
            name: "Sleep Now in Sapphire",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        742 => DutyInfo {
            name: "The Diadem",
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        743 => DutyInfo {
            name: "Faded Memories",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        745 => DutyInfo {
            name: "The Shifting Oubliettes of Lyhe Ghiah",
            high_end: false,
            content_kind: ContentKind::TreasureHunt,
        },
        746 => DutyInfo {
            name: "Matoya's Relict",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        747 => DutyInfo {
            name: "Eden's Promise: Litany",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        748 => DutyInfo {
            name: "Eden's Promise: Litany (Savage)",
            high_end: true,
            content_kind: ContentKind::Raids,
        },
        749 => DutyInfo {
            name: "Eden's Promise: Umbra",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        750 => DutyInfo {
            name: "Eden's Promise: Umbra (Savage)",
            high_end: true,
            content_kind: ContentKind::Raids,
        },
        751 => DutyInfo {
            name: "Eden's Promise: Anamorphosis",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        752 => DutyInfo {
            name: "Eden's Promise: Anamorphosis (Savage)",
            high_end: true,
            content_kind: ContentKind::Raids,
        },
        753 => DutyInfo {
            name: "The Diadem",
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        754 => DutyInfo {
            name: "Anything Gogo's",
            high_end: false,
            content_kind: ContentKind::Other(27),
        },
        755 => DutyInfo {
            name: "Triple Triad Open Tournament",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        756 => DutyInfo {
            name: "Triple Triad Invitational Parlor",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        758 => DutyInfo {
            name: "Eden's Promise: Eternity",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        759 => DutyInfo {
            name: "Eden's Promise: Eternity (Savage)",
            high_end: true,
            content_kind: ContentKind::Raids,
        },
        760 => DutyInfo {
            name: "Delubrum Reginae",
            high_end: false,
            content_kind: ContentKind::Other(29),
        },
        761 => DutyInfo {
            name: "Delubrum Reginae (Savage)",
            high_end: false,
            content_kind: ContentKind::Other(29),
        },
        762 => DutyInfo {
            name: "Castrum Marinum",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        763 => DutyInfo {
            name: "Castrum Marinum (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        764 => DutyInfo {
            name: "The Great Ship Vylbrand",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        765 => DutyInfo {
            name: "Fit for a Queen",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        766 => DutyInfo {
            name: "Novice Mahjong (Quick Ranked Match)",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        767 => DutyInfo {
            name: "Advanced Mahjong (Quick Ranked Match)",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        768 => DutyInfo {
            name: "Four-player Mahjong (Quick Match, Kuitan Enabled)",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        769 => DutyInfo {
            name: "Four-player Mahjong (Quick Match, Kuitan Disabled)",
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        770 => DutyInfo {
            name: "Ocean Fishing",
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        771 => DutyInfo {
            name: "Ocean Fishing",
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        772 => DutyInfo {
            name: "Ocean Fishing",
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        773 => DutyInfo {
            name: "Ocean Fishing",
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        774 => DutyInfo {
            name: "Ocean Fishing",
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        775 => DutyInfo {
            name: "Ocean Fishing",
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        776 => DutyInfo {
            name: "The Whorleater (Unreal)",
            high_end: true,
            content_kind: ContentKind::Trials,
        },
        777 => DutyInfo {
            name: "Paglth'an",
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        778 => DutyInfo {
            name: "Zadnor",
            high_end: false,
            content_kind: ContentKind::Other(29),
        },
        779 => DutyInfo {
            name: "The Tower at Paradigm's Breach",
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        780 => DutyInfo {
            name: "Death Unto Dawn",
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        781 => DutyInfo {
            name: "The Cloud Deck",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        782 => DutyInfo {
            name: "The Cloud Deck (Extreme)",
            high_end: false,
            content_kind: ContentKind::Trials,
        },
    };
}
