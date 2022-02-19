use clap::Parser;
use maga::shortbool;
use minivec::MiniVec;
use rand::{prelude::ThreadRng, Rng};
use serde::{Deserialize, Serialize};

const DATA: &[u8] = include_bytes!("./tweet.json");

/// Random Donald Tweets
fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let mut rng: ThreadRng = rand::thread_rng();
    let db: MiniVec<DonaldTweet> = serde_json::from_slice(DATA)?;
    let db: MiniVec<&DonaldTweet> = db
        .iter()
        .filter(|t| {
            !t.is_retweet
                && !t.text.starts_with("RT")
                && !t.text.contains('@')
                && !t.text.starts_with(r#""""#)
        })
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
