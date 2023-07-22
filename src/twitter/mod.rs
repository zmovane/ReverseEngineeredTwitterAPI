mod auth;
mod search;
mod types;
use dotenv::dotenv;
use reqwest::Client;

pub struct API {
    client: Client,
    guest_token: String,
    csrf_token: String,
}

#[cfg(test)]
mod tests {
    use super::{types::Data, *};

    async fn login(api: &mut API) -> Result<String, String> {
        dotenv().ok();
        let name = std::env::var("TWITTER_USER_NAME").unwrap();
        let pwd = std::env::var("TWITTER_USER_PASSWORD").unwrap();
        api.login(name, pwd, "".to_string()).await
    }

    async fn search(api: &mut API) -> Result<Data, reqwest::Error> {
        let content = "@shareverse_bot -filter:retweets".to_string();
        let limit = 50;
        let cursor = "".to_string();
        api.search(content, limit, cursor).await
    }

    #[tokio::test]
    async fn test_login() {
        let mut api = API::new();
        let loggined = login(&mut api).await;
        assert!(loggined.is_ok());

        let is_logged_in = api.is_logged_in().await;
        assert!(is_logged_in);

        let result = search(&mut api).await;
        assert!(result.is_ok());
    }
}
