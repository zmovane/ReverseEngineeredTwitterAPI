use cmd::Cmd;
use cmd::NFTArgs;
use dotenv::dotenv;
use ethers::types::H160;
use ethers::types::U64;
use log::error;
use prisma::tweets;
use prisma::CommandType;
use prisma::PrismaClient;
use prisma_client_rust::Direction;
use regex::Regex;
use reverse_engineered_twitter_api::types::Tweet;
use reverse_engineered_twitter_api::ReAPI;
use std::str::FromStr;
use std::time::Duration;
use tokio::join;

mod cmd;
#[allow(warnings, unused)]
mod prisma;
mod util;

async fn save_tweet(db_client: &PrismaClient, command: &str, tweet: Tweet) {
    db_client
        .tweets()
        .upsert(
            tweets::id::equals(tweet.id.clone()),
            tweets::create(
                tweet.id,
                tweet.user_id,
                tweet.text.to_owned(),
                command.to_string(),
                tweet.permanent_url,
                CommandType::Nft,
                tweet.time_parsed,
                vec![],
            ),
            vec![
                tweets::text::set(tweet.text.to_owned()),
                tweets::command::set(command.to_string()),
            ],
        )
        .exec()
        .await
        .unwrap();
}

async fn fetch_tweets(db_client: &PrismaClient) {
    let mut twitter_api = ReAPI::new();
    let name = std::env::var("TWITTER_USER_NAME").unwrap();
    let pwd = std::env::var("TWITTER_USER_PASSWORD").unwrap();
    let _ = twitter_api.login(&name, &pwd, "").await;

    // ensure account logged in
    let logged_in = twitter_api.is_logged_in().await;
    if !logged_in {
        panic!("failed to login")
    }
    let pattern: Regex = util::command_pattern();
    let q = "@shareverse_bot";
    let limit = 50;
    let mut cursor = String::from("");
    loop {
        let res = twitter_api.search_tweets(q, limit, &cursor).await;
        match res {
            Ok((tweets, next_cursor)) => {
                for tweet in tweets {
                    let text = tweet.text.to_owned();
                    let matched = pattern.is_match(&text);
                    if !matched {
                        continue;
                    }
                    let command = pattern.captures(&text).unwrap().get(0).unwrap().as_str();
                    save_tweet(db_client, command, tweet).await;
                }
                cursor = next_cursor;
            }
            Err(e) => {
                error!("{}", e);
            }
        }
        tokio::time::sleep(Duration::from_secs(300)).await;
    }
}

async fn mint_nft(db_client: &PrismaClient, cmd: &Cmd, tw: tweets::Data) {
    let pattern: Regex = util::command_pattern();
    let cmd_group = pattern.captures(&tw.command).unwrap();
    let cmd_img_url = cmd_group.get(2).unwrap().as_str();
    let cmd_address = cmd_group.get(4).unwrap().as_str();
    let args = NFTArgs {
        to: H160::from_str(cmd_address).unwrap(),
        image: cmd_img_url.to_string(),
        name: tw.user_id,
        description: "from @shareverse_bot".to_string(),
    };
    let result = cmd.mint_nft(args).await;
    match result {
        Ok(tx) => {
            let tx = tx.unwrap();
            if tx.status.is_some() && tx.status.unwrap().eq(&U64::one()) {
                let _ = db_client
                    .tweets()
                    .update(
                        tweets::id::equals(tw.id),
                        vec![
                            tweets::excuted::set(true),
                            tweets::excuted_date::set(chrono::offset::Utc::now().fixed_offset()),
                            tweets::excuted_tx::set(tx.transaction_hash.to_string()),
                        ],
                    )
                    .exec()
                    .await;
            }
        }
        Err(_) => {}
    }
}

async fn excute_commands(db_client: &PrismaClient) {
    let client = util::new_client();
    let cmd = Cmd::new(client);
    loop {
        let tweet_lst: Vec<tweets::Data> = db_client
            .tweets()
            .find_many(vec![tweets::excuted::equals(false)])
            .order_by(tweets::date::order(Direction::Desc))
            .exec()
            .await
            .unwrap();
        for tw in tweet_lst {
            mint_nft(db_client, &cmd, tw).await;
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client: PrismaClient = PrismaClient::_builder().build().await.unwrap();
    let future_fetch_tweets = fetch_tweets(&client);
    let future_excute_commands = excute_commands(&client);
    join!(future_excute_commands, future_fetch_tweets);
}
