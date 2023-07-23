use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Http, Provider},
    signers::{Signer, Wallet},
};
use regex::Regex;
use std::{sync::Arc, time::Duration};

pub fn new_client() -> Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    let pk: String = std::env::var("PK").unwrap().to_string();
    let rpc_url: String = std::env::var("RPC_URL").unwrap().to_string();
    let wallet: Wallet<SigningKey> = pk.parse().unwrap();
    let provider: Provider<Http> = Provider::<Http>::try_from(rpc_url)
        .unwrap()
        .interval(Duration::from_millis(10u64));
    let wallet = wallet.with_chain_id(5u64);
    Arc::new(SignerMiddleware::new(provider, wallet))
}

pub fn command_pattern() -> Regex {
    let pattern_img_url =
        r#"(https://t.co/[a-zA-Z0-9]{10})|(http(s?):([/|.|\w|\s|-])*.(?:jpe?g|gif|png|svg|webp))"#;
    let pattern_address = r#"0x[a-f0-9]{40}"#;
    let pattern_mint_cmd = format!(
        r#"@shareverse_bot\s+(mint|MINT)?\s+({})(to|TO)?\s+({})"#,
        pattern_img_url, pattern_address
    );
    Regex::new(&pattern_mint_cmd).unwrap()
}
