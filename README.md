[![crates.io](https://img.shields.io/crates/v/reverse-engineered-twitter-api.svg)](https://crates.io/crates/reverse-engineered-twitter-api)
[![Downloads crates.io](https://img.shields.io/crates/d/reverse-engineered-twitter-api.svg?label=crates.io%20downloads)](https://crates.io/crates/reverse-engineered-twitter-api)

# ReverseEngineeredTwitterAPI

Reverse engineered Twitter's API has not a lots of limitations, the rate limit is consistent with regular users logging in through the twitter website, and there is no need to consider payment issues.

## Installation

```
[dependencies]
reverse-engineered-twitter-api = "0.1.1"
```

## Usage

#### Login

```
let name = std::env::var("TWITTER_USER_NAME").unwrap();
let pwd = std::env::var("TWITTER_USER_PASSWORD").unwrap();

// If no verification code is required, set it to empty
let confirmation_code = "";
api.login(&name, &pwd, confirmation_code).await

// check if account is logged in
let is_logged_in = api.is_logged_in().await;
```

#### Search

```
// search tweets
let content = "@lidangzzz -filter:retweets";
let limit = 50;
let cursor = "";
api.search_tweets(content, limit, cursor).await;
```
