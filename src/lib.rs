/// Deserialize / Serialize short bool
/// e.g 't' becomes true, while 'f' becomes false
pub mod shortbool {
    use serde::{de, Deserializer, Serializer};
    use std::fmt;

    pub fn serialize<S>(value: &bool, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if *value {
            ser.serialize_str("t")
        } else {
            ser.serialize_str("f")
        }
    }

    pub fn deserialize<'de, D>(de: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ShortBoolVisitor;
        impl<'de> de::Visitor<'de> for ShortBoolVisitor {
            type Value = bool;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a boolean value")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "t" | "true" => Ok(true),
                    "f" | "false" => Ok(false),
                    _ => Err(E::custom(format!("found `{}`, expected `t` or `f`", value))),
                }
            }
        }

        de.deserialize_str(ShortBoolVisitor)
    }
}
