use sestring::SeString;
use serde::{Deserializer, Deserialize, Serializer, Serialize};

pub fn deserialize<'de, D>(de: D) -> Result<SeString, D::Error>
    where D: Deserializer<'de>,
{
    let b64 = String::deserialize(de)?;
    use base64::Engine;
    let engine = base64::engine::general_purpose::GeneralPurpose::new(
        &base64::alphabet::STANDARD,
        base64::engine::general_purpose::GeneralPurposeConfig::new()
            .with_decode_padding_mode(base64::engine::DecodePaddingMode::Indifferent),
    );
    let bytes = engine.decode(&b64).map_err(|e| serde::de::Error::custom(format!("invalid base64: {:?}", e)))?;
    SeString::parse(&bytes).map_err(|e| serde::de::Error::custom(format!("invalid sestring: {:?}", e)))
}

pub fn serialize<S>(sestring: &SeString, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
{
    let bytes = sestring.encode();
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(&bytes).serialize(ser)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use sestring::SeString;

    #[derive(Deserialize, Serialize)]
    struct Wrapped(#[serde(with = "super")] SeString);

    #[test]
    fn preserves_standard_base64_wire_format() {
        let value: Wrapped = serde_json::from_str(r#""VGVzdA==""#).unwrap();
        assert_eq!(serde_json::to_string(&value).unwrap(), r#""VGVzdA==""#);
    }

    #[test]
    fn rejects_invalid_base64() {
        assert!(serde_json::from_str::<Wrapped>(r#""not base64!""#).is_err());
    }

    #[test]
    fn accepts_legacy_unpadded_base64() {
        let value: Wrapped = serde_json::from_str(r#""VGVzdA""#).unwrap();
        assert_eq!(serde_json::to_string(&value).unwrap(), r#""VGVzdA==""#);
    }
}
