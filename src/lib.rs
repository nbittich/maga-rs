use minivec::MiniVec;
use rand::{prelude::ThreadRng, Rng};
use serde::{Deserialize, Serialize};

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

const DATA: &[u8] = include_bytes!("./tweet.json");

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DonaldTweet<'a> {
    id: u64,
    text: String,
    #[serde(with = "shortbool")]
    is_retweet: bool,
    #[serde(with = "shortbool")]
    is_deleted: bool,
    #[serde(with = "shortbool")]
    is_flagged: bool,
    device: &'a str,
    favorites: u32,
    retweets: u32,
    date: &'a str,
}
impl<'a> DonaldTweet<'a> {
    pub fn text(&self) -> &str {
        &self.text
    }
}
pub struct RandomTweetGenerator<'a> {
    db: MiniVec<DonaldTweet<'a>>,
}
impl Default for RandomTweetGenerator<'_> {
    fn default() -> Self {
        let db: MiniVec<DonaldTweet> = serde_json::from_slice(DATA).unwrap();
        let db: MiniVec<DonaldTweet> = db
            .into_iter()
            .filter(|t| {
                !t.is_retweet
                    && !t.text.starts_with("RT")
                    && !t.text.contains('@')
                    && !t.text.starts_with(r#""""#)
            })
            .collect();
        Self { db }
    }
}

impl<'a> RandomTweetGenerator<'a> {
    pub fn get_random_tweet(&self) -> &DonaldTweet<'a> {
        let mut rng: ThreadRng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.db.len());
        &self.db[random_index]
    }
}
