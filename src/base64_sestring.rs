use sestring::SeString;
use serde::{Deserializer, Deserialize, Serializer, Serialize};

pub fn deserialize<'de, D>(de: D) -> Result<SeString, D::Error>
    where D: Deserializer<'de>,
{
    let b64 = String::deserialize(de)?;
    let bytes = base64::decode(&b64).map_err(|e| serde::de::Error::custom(format!("invalid base64: {:?}", e)))?;
    SeString::parse(&bytes).map_err(|e| serde::de::Error::custom(format!("invalid sestring: {:?}", e)))
}

pub fn serialize<S>(sestring: &SeString, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
{
    let bytes = sestring.encode();
    base64::encode(&bytes).serialize(ser)
}
