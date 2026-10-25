use std::collections::HashMap;

/// One row of the client's LFGExtensionContent sheet (the PF extension list).
/// LfgContent mirrors the client's Agent::ContentsType resolution (None/Roulette/Regular);
/// it is not named ContentsType to avoid clashing with the ContentType sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LfgContent {
    /// ContentFinderCondition row id
    Regular(u32),
    /// ContentRoulette row id
    Roulette(u8),
    /// row type the client cannot resolve to a specific duty
    Unresolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LfgExtensionContent {
    /// 0-based LFGExtensionContent row id (the packet's DutyId = row id + 1)
    pub row_id: u32,
    /// sheet group byte (3 = gold saucer list, 4 = deep dungeon list)
    pub group: u8,
    pub content: LfgContent,
}

/// Keyed by the packet's DutyId (Detailed.DutyId = row id + 1), as written by the client.
lazy_static::lazy_static! {
    pub static ref LFG_EXTENSION_CONTENT: HashMap<u32, LfgExtensionContent> = maplit::hashmap! {
        1 => LfgExtensionContent { row_id: 0, group: 1, content: LfgContent::Regular(721) },
        2 => LfgExtensionContent { row_id: 1, group: 1, content: LfgContent::Regular(722) },
        3 => LfgExtensionContent { row_id: 2, group: 2, content: LfgContent::Regular(283) },
        4 => LfgExtensionContent { row_id: 3, group: 2, content: LfgContent::Regular(581) },
        5 => LfgExtensionContent { row_id: 4, group: 2, content: LfgContent::Regular(598) },
        6 => LfgExtensionContent { row_id: 5, group: 2, content: LfgContent::Regular(639) },
        7 => LfgExtensionContent { row_id: 6, group: 2, content: LfgContent::Regular(735) },
        8 => LfgExtensionContent { row_id: 7, group: 2, content: LfgContent::Regular(760) },
        9 => LfgExtensionContent { row_id: 8, group: 2, content: LfgContent::Regular(761) },
        10 => LfgExtensionContent { row_id: 9, group: 2, content: LfgContent::Regular(778) },
        11 => LfgExtensionContent { row_id: 10, group: 3, content: LfgContent::Unresolved },
        12 => LfgExtensionContent { row_id: 11, group: 3, content: LfgContent::Roulette(21) },
        13 => LfgExtensionContent { row_id: 12, group: 3, content: LfgContent::Roulette(18) },
        14 => LfgExtensionContent { row_id: 13, group: 3, content: LfgContent::Roulette(19) },
        15 => LfgExtensionContent { row_id: 14, group: 3, content: LfgContent::Roulette(20) },
        16 => LfgExtensionContent { row_id: 15, group: 3, content: LfgContent::Roulette(25) },
        17 => LfgExtensionContent { row_id: 16, group: 3, content: LfgContent::Roulette(22) },
        18 => LfgExtensionContent { row_id: 17, group: 3, content: LfgContent::Roulette(23) },
        19 => LfgExtensionContent { row_id: 18, group: 3, content: LfgContent::Roulette(24) },
        20 => LfgExtensionContent { row_id: 19, group: 3, content: LfgContent::Regular(195) },
        21 => LfgExtensionContent { row_id: 20, group: 3, content: LfgContent::Regular(756) },
        22 => LfgExtensionContent { row_id: 21, group: 3, content: LfgContent::Regular(199) },
        23 => LfgExtensionContent { row_id: 22, group: 3, content: LfgContent::Regular(645) },
        24 => LfgExtensionContent { row_id: 23, group: 3, content: LfgContent::Regular(650) },
        25 => LfgExtensionContent { row_id: 24, group: 3, content: LfgContent::Regular(768) },
        26 => LfgExtensionContent { row_id: 25, group: 3, content: LfgContent::Regular(769) },
        27 => LfgExtensionContent { row_id: 26, group: 3, content: LfgContent::Regular(958) },
        28 => LfgExtensionContent { row_id: 27, group: 2, content: LfgContent::Regular(1018) },
        29 => LfgExtensionContent { row_id: 28, group: 4, content: LfgContent::Regular(174) },
        30 => LfgExtensionContent { row_id: 29, group: 4, content: LfgContent::Regular(540) },
        31 => LfgExtensionContent { row_id: 30, group: 4, content: LfgContent::Regular(897) },
        32 => LfgExtensionContent { row_id: 31, group: 4, content: LfgContent::Regular(1032) },
        33 => LfgExtensionContent { row_id: 32, group: 4, content: LfgContent::Regular(1063) },
        34 => LfgExtensionContent { row_id: 33, group: 4, content: LfgContent::Regular(1065) },
        35 => LfgExtensionContent { row_id: 34, group: 3, content: LfgContent::Regular(1095) },
        36 => LfgExtensionContent { row_id: 35, group: 2, content: LfgContent::Regular(1093) },
        37 => LfgExtensionContent { row_id: 36, group: 2, content: LfgContent::Regular(1114) },
    };
}
