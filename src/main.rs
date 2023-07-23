use dotenv::dotenv;
use ethers::types::H160;
use log::{error, info};
use regex::Regex;
use reverse_engineered_twitter_api::ReAPI;
use std::str::FromStr;
use tokio::join;
mod cmd;
mod util;

async fn fetch_tweets() {
    let mut twitter_api = ReAPI::new();
    let name = std::env::var("TWITTER_USER_NAME").unwrap();
    let pwd = std::env::var("TWITTER_USER_PASSWORD").unwrap();
    let _ = twitter_api.login(&name, &pwd, "").await;

    // ensure account logged in
    let logged_in = twitter_api.is_logged_in().await;
    if !logged_in {
        panic!("failed to login")
    }
    let pattern_img_url =
        r#"(https://t.co/[a-zA-Z0-9]{10})|(http(s?):([/|.|\w|\s|-])*.(?:jpe?g|gif|png|svg|webp))"#;
    let pattern_address = r#"0x[a-f0-9]{40}"#;
    let pattern_mint_cmd = format!(
        r#"@shareverse_bot\s+(mint|MINT)?\s+{}(to|TO)?\s+{}"#,
        pattern_img_url, pattern_address
    );
    let pattern = Regex::new(&pattern_mint_cmd).unwrap();
    let q = "@shareverse_bot -filter:retweets";
    let limit = 10;
    let mut cursor = String::from("");
    loop {
        let res = twitter_api.search_tweets(q, limit, &cursor).await;
        match res {
            Ok((tweets, next_cursor)) => {
                for tweet in tweets {
                    if pattern.is_match(&tweet.text) {
                        // TODO save to db
                    }
                }
                cursor = next_cursor;
            }
            Err(_) => {}
        }
    }
}

// TODO: excute commands
async fn excute_commands() {
    let client = util::new_client();
    let command = cmd::NFTMinting {
        to: H160::from_str("0x8a35A64A20840c71d2eFb5aAeEF6933F5e6A3047").unwrap(),
        image: "https://imart-nft.s3.us-east-1.amazonaws.com/imart/1684834976.json".to_string(),
        name: "a tiny boat".to_string(),
        description: "from Shareverse Bot".to_string(),
    };

    let result = command.excute(client).await;
    match result {
        Ok(_) => {
            info!("succeed!")
        }
        Err(e) => {
            error!("error: {}", e)
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let future_fetch_tweets = fetch_tweets();
    let future_excute_commands = excute_commands();
    join!(future_excute_commands, future_fetch_tweets);
}
