use askama::Template;
use crate::listing_container::QueriedListing;
use std::borrow::Borrow;
use crate::ffxiv::Language;
use crate::sestring_ext::SeStringExt;
use crate::listing::PartyFinderCategory;

#[derive(Debug, Template)]
#[template(path = "listings.html")]
pub struct ListingsTemplate {
    pub containers: Vec<QueriedListing>,
    pub lang: Language,
}
