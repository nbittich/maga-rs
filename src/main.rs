use rand::{prelude::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use clap::Parser;



const DATA:&[u8] = include_bytes!("./tweet.json");

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let mut rng: ThreadRng = rand::thread_rng();
    let db: Vec<DonaldTweet> = serde_json::from_slice(DATA)?;
    let db: Vec<&DonaldTweet> = db.iter().filter(|t| !t.is_retweet 
        && !t.text.starts_with("RT")
        && !t.text.contains("@")
        && !t.text.starts_with(r#""""#))
    .collect();
    for _ in 0..args.count {
        let random_index = rng.gen_range(0..db.len());
        unsafe {
            println!("{}", db.get_unchecked(random_index).text);
        }
    }

    Ok(())
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] 
struct DonaldTweet<'a> {
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


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of tweets to display
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}


mod shortbool {
    use serde::{de, Serializer, Deserializer};
    use std::fmt;

    pub fn serialize<S>(value: &bool, ser: S) -> Result<S::Ok, S::Error> where S: Serializer {
        if *value {
            ser.serialize_str("t")
        } else {
            ser.serialize_str("f")
        }
    } 
    
    pub fn deserialize<'de, D>(de: D) -> Result<bool, D::Error> where D: Deserializer<'de> {
    
        struct ShortBoolVisitor;
        impl<'de> de::Visitor<'de> for ShortBoolVisitor {
            type Value = bool;
            
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a boolean value")
            }
            
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> 
                where E: de::Error,
            {
                match value {
                    "t" => Ok(true),
                    "f" => Ok(false),
                    _ => Err(E::custom(format!("found `{}`, expected `t` or `f`", value))),
                }
            }
        }
        
        
        de.deserialize_str(ShortBoolVisitor)
    }
}