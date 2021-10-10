use askama::Template;
use crate::ffxiv::Language;
use crate::stats::Statistics;

#[derive(Debug, Template)]
#[template(path = "stats.html")]
pub struct StatsTemplate {
    pub stats: Statistics,
    pub lang: Language,
}
