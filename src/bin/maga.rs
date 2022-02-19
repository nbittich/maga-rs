use clap::Parser;
use maga::RandomTweetGenerator;

/// Random Donald Tweets
fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let tweet_generator: RandomTweetGenerator = Default::default();
    for _ in 0..args.count {
        println!("{}", tweet_generator.get_random_tweet().text());
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of tweets to display
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}
