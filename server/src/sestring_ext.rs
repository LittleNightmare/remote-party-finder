use sestring::{Payload, SeString};

pub trait SeStringExt {
    fn full_text(&self) -> String;
}

impl SeStringExt for SeString {
    fn full_text(&self) -> String {
        self.0.iter()
            .flat_map(|payload| {
                match payload {
                    Payload::Text(t) => Some(&*t.0),
                    Payload::AutoTranslate(at) => crate::ffxiv::AUTO_TRANSLATE
                        .get(&(u32::from(at.group), at.key))
                        .map(std::ops::Deref::deref),
                    _ => None,
                }
            })
            .collect()
    }
}
