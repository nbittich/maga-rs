#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use clap::Parser;
use maga::{Tweet, RandomTweetGenerator};

#[get("/?<count>")]
fn index(count: Option<u16>) -> String {
    let mut result: Vec<&Tweet> = vec![];
    let tweet_generator: RandomTweetGenerator = Default::default();

    let count = if let Some(count) = count { count } else { 1 };
    for _ in 0..count {
        let tweet = &tweet_generator.get_random_tweet();
        result.push(tweet);
    }
    serde_json::to_string_pretty(&result).unwrap()
}

fn main() {
    let args = Args::parse();
    let mut conf: rocket::Config = rocket::Config::production();
    conf.port = args.port;
    rocket::custom(conf).mount("/", routes![index]).launch();
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Port
    #[clap(short, long, default_value_t = 8000)]
    port: u16,
}
