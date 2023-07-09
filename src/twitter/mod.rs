mod api;
pub use self::api::API;
use dotenv::dotenv;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_login() {
        dotenv().ok();
        let name = std::env::var("TWITTER_USER_NAME").unwrap();
        let pwd = std::env::var("TWITTER_USER_PASSWORD").unwrap();
        let mut api = API::new();
        let result = api.login(name, pwd, "".to_string()).await;
        assert!(result.is_ok())
    }
}
