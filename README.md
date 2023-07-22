[![crates.io](https://img.shields.io/crates/v/reverse-engineered-twitter-api.svg)](https://crates.io/crates/reverse-engineered-twitter-api)
[![Downloads crates.io](https://img.shields.io/crates/d/reverse-engineered-twitter-api.svg?label=crates.io%20downloads)](https://crates.io/crates/reverse-engineered-twitter-api)

# ReverseEngineeredTwitterAPI

Reverse engineered Twitter's API has not a lots of limitations, there is no need to worry about rate limit and payment requirements.

## Installation

```
[dependencies]
reverse-engineered-twitter-api = "0.1.0"
```

## Usage

```
// login
let name = std::env::var("TWITTER_USER_NAME").unwrap();
let pwd = std::env::var("TWITTER_USER_PASSWORD").unwrap();
let confirmation_code = String::from(""); // optional
api.login(name, pwd, confirmation_code).await

// check if account is logged in
let is_logged_in = api.is_logged_in().await;

// search tweets
let content = "@shareverse_bot -filter:retweets".to_string();
let limit = 50;
let cursor = String::from("");
api.search_tweets(content, limit, cursor).await
```
