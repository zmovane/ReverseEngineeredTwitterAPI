use std::str::FromStr;

use dotenv::dotenv;
use ethers::types::H160;
use log::{error, info};

mod cmd;
mod twitter;
mod util;

#[tokio::main]
async fn main() {
    dotenv().ok();
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
