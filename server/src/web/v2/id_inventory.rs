use ffxiv_types_cn::{jobs::ClassJob, Role};

use crate::{
    ffxiv,
    listing::{ConditionFlags, DutyCategory, DutyType, JobFlags, LootRuleFlags, ObjectiveFlags},
};

pub const PHASE_ONE_PRIMARY_EXPOSES_DATACENTER: bool = false;

pub const ROLE_ID_DPS: u32 = 1;
pub const ROLE_ID_HEALER: u32 = 2;
pub const ROLE_ID_TANK: u32 = 3;

pub const CATEGORY_IDS: [u32; 16] = [
    0, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768,
];

pub const DUTY_TYPE_IDS: [u32; 3] = [0, 1, 2];

pub const ROLE_IDS: [u32; 3] = [ROLE_ID_DPS, ROLE_ID_HEALER, ROLE_ID_TANK];
pub const OBJECTIVE_IDS: [u32; 3] = [
    ObjectiveFlags::DUTY_COMPLETION.bits(),
    ObjectiveFlags::PRACTICE.bits(),
    ObjectiveFlags::LOOT.bits(),
];
pub const CONDITION_IDS: [u32; 3] = [
    ConditionFlags::DUTY_COMPLETE.bits(),
    ConditionFlags::DUTY_INCOMPLETE.bits(),
    ConditionFlags::DUTY_COMPLETE_WEEKLY_REWARD_UNCLAIMED.bits(),
];
pub const LOOT_RULE_IDS: [u32; 4] = [
    LootRuleFlags::NONE.bits(),
    LootRuleFlags::GREED_ONLY.bits(),
    LootRuleFlags::LOOTMASTER.bits(),
    LootRuleFlags::GREED_ONLY.bits() | LootRuleFlags::LOOTMASTER.bits(),
];

pub fn world_ids() -> Vec<u32> {
    sorted_ids(ffxiv::WORLDS.keys().copied().collect())
}

pub fn world_id(world_id: u16) -> u32 {
    u32::from(world_id)
}

pub fn category_id(category: DutyCategory) -> u32 {
    category.as_u32()
}

pub fn duty_id(duty_id: u16) -> u32 {
    u32::from(duty_id)
}

pub fn duty_type_id(duty_type: DutyType) -> u32 {
    u32::from(duty_type.as_u8())
}

pub fn job_ids() -> Vec<u32> {
    accepted_job_ids(JobFlags::all())
}

pub fn accepted_job_flag_bits(job_id: u32) -> Option<u32> {
    accepted_job_flag(job_id).map(|job_flag| job_flag.bits())
}

pub fn slot_accepts_job_id(flags: JobFlags, job_id: u32) -> bool {
    accepted_job_flag(job_id).is_some_and(|job_flag| flags.intersects(job_flag))
}

pub fn filled_job_id(job_id: u8) -> Option<u32> {
    let job_id = u32::from(job_id);
    ffxiv::JOBS.contains_key(&job_id).then_some(job_id)
}

pub fn accepted_job_ids(flags: JobFlags) -> Vec<u32> {
    flags
        .classjobs()
        .into_iter()
        .filter_map(job_id_for_classjob)
        .collect()
}

pub fn role_id(role: Role) -> u32 {
    match role {
        Role::Dps => ROLE_ID_DPS,
        Role::Healer => ROLE_ID_HEALER,
        Role::Tank => ROLE_ID_TANK,
    }
}

pub fn role_id_for_job_id(job_id: u32) -> Option<u32> {
    ffxiv::JOBS.get(&job_id)?.role().map(role_id)
}

pub fn role_id_for_job_ids(job_ids: &[u32]) -> Option<u32> {
    let mut role_ids = job_ids
        .iter()
        .filter_map(|job_id| role_id_for_job_id(*job_id));
    let first = role_ids.next()?;
    role_ids.all(|role_id| role_id == first).then_some(first)
}

pub fn objective_ids(flags: ObjectiveFlags) -> Vec<u32> {
    flag_ids(flags.bits(), &OBJECTIVE_IDS)
}

pub fn condition_ids(flags: ConditionFlags) -> Vec<u32> {
    flag_ids(flags.bits() & !ConditionFlags::NONE.bits(), &CONDITION_IDS)
}

pub fn loot_rule_id(flags: LootRuleFlags) -> u32 {
    flags.bits()
}

fn job_id_for_classjob(class_job: ClassJob) -> Option<u32> {
    ffxiv::JOBS
        .iter()
        .find_map(|(job_id, candidate)| (*candidate == class_job).then_some(*job_id))
}

fn accepted_job_flag(job_id: u32) -> Option<JobFlags> {
    all_job_flags().into_iter().find(
        |flag| matches!(accepted_job_ids(*flag).as_slice(), [candidate] if *candidate == job_id),
    )
}

fn all_job_flags() -> [JobFlags; 31] {
    [
        JobFlags::GLADIATOR,
        JobFlags::PUGILIST,
        JobFlags::MARAUDER,
        JobFlags::LANCER,
        JobFlags::ARCHER,
        JobFlags::CONJURER,
        JobFlags::THAUMATURGE,
        JobFlags::PALADIN,
        JobFlags::MONK,
        JobFlags::WARRIOR,
        JobFlags::DRAGOON,
        JobFlags::BARD,
        JobFlags::WHITE_MAGE,
        JobFlags::BLACK_MAGE,
        JobFlags::ARCANIST,
        JobFlags::SUMMONER,
        JobFlags::SCHOLAR,
        JobFlags::ROGUE,
        JobFlags::NINJA,
        JobFlags::MACHINIST,
        JobFlags::DARK_KNIGHT,
        JobFlags::ASTROLOGIAN,
        JobFlags::SAMURAI,
        JobFlags::RED_MAGE,
        JobFlags::BLUE_MAGE,
        JobFlags::GUNBREAKER,
        JobFlags::DANCER,
        JobFlags::REAPER,
        JobFlags::SAGE,
        JobFlags::VIPER,
        JobFlags::PICTOMANCER,
    ]
}

fn flag_ids(flags: u32, known_ids: &[u32]) -> Vec<u32> {
    known_ids
        .iter()
        .copied()
        .filter(|id| flags & id != 0)
        .collect()
}

fn sorted_ids(mut ids: Vec<u32>) -> Vec<u32> {
    ids.sort_unstable();
    ids
}
