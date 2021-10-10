use sestring::{Payload, SeString};
use crate::ffxiv::Language;

pub trait SeStringExt {
    fn full_text(&self, lang: &Language) -> String;
}

impl SeStringExt for SeString {
    fn full_text(&self, lang: &Language) -> String {
        self.0.iter()
            .flat_map(|payload| {
                match payload {
                    Payload::Text(t) => Some(&*t.0),
                    Payload::AutoTranslate(at) => crate::ffxiv::AUTO_TRANSLATE
                        .get(&(u32::from(at.group), at.key))
                        .map(|text| text.text(lang)),
                    _ => None,
                }
            })
            .collect()
    }
}
