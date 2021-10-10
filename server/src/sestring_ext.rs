use sestring::{Payload, SeString};

pub trait SeStringExt {
    fn full_text(&self, codes: &str) -> String;
}

impl SeStringExt for SeString {
    fn full_text(&self, codes: &str) -> String {
        self.0.iter()
            .flat_map(|payload| {
                match payload {
                    Payload::Text(t) => Some(&*t.0),
                    Payload::AutoTranslate(at) => crate::ffxiv::AUTO_TRANSLATE
                        .get(&(u32::from(at.group), at.key))
                        .map(|text| text.from_codes(codes)),
                    _ => None,
                }
            })
            .collect()
    }
}
