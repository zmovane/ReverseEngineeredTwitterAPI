mod api;

pub use self::api::Account;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_login() {
        let account = Account {};
        let result = account
            .login(
                std::env::var("TWITTER_USER_NAME").unwrap(),
                std::env::var("TWITTER_USER_PASSWORD").unwrap(),
                "".to_owned(),
            )
            .await;
        assert!(result.is_ok())
    }
}
