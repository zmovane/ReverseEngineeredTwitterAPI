use std::error::Error;

use async_trait::async_trait;
use serde_json::json;

use crate::{
    types_resp::{followers_types::FollowersResp, following_types::FollowingResp},
    ReAPI, BEARER_TOKEN,
};

#[async_trait]
pub trait Relation {
    async fn get_followers(&self, uid: &String) -> Result<FollowersResp, Box<dyn Error>>;
    async fn get_following(&self, uid: &String) -> Result<FollowingResp, Box<dyn Error>>;
}

#[async_trait]
impl Relation for ReAPI {
    async fn get_followers(&self, uid: &String) -> Result<FollowersResp, Box<dyn Error>> {
        let variables = json!(
            {"userId":uid.as_str(),"count":20,"includePromotedContent":false}
        );
        let features = json!(
            {"responsive_web_graphql_exclude_directive_enabled":true,"verified_phone_label_enabled":false,"responsive_web_home_pinned_timelines_enabled":true,"creator_subscriptions_tweet_preview_api_enabled":true,"responsive_web_graphql_timeline_navigation_enabled":true,"responsive_web_graphql_skip_user_profile_image_extensions_enabled":false,"c9s_tweet_anatomy_moderator_badge_enabled":true,"tweetypie_unmention_optimization_enabled":true,"responsive_web_edit_tweet_api_enabled":true,"graphql_is_translatable_rweb_tweet_is_translatable_enabled":true,"view_counts_everywhere_api_enabled":true,"longform_notetweets_consumption_enabled":true,"responsive_web_twitter_article_tweet_consumption_enabled":false,"tweet_awards_web_tipping_enabled":false,"freedom_of_speech_not_reach_fetch_enabled":true,"standardized_nudges_misinfo":true,"tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled":true,"longform_notetweets_rich_text_read_enabled":true,"longform_notetweets_inline_media_enabled":true,"responsive_web_media_download_video_enabled":false,"responsive_web_enhance_cards_enabled":false}
        );
        // variables["product"] = "Latest".into();
        let q = [
            ("variables", variables.to_string()),
            ("features", features.to_string()),
        ];
        let req = self
            .client
            .get("https://twitter.com/i/api/graphql/9LlZicVr2IBf4u2qW5n4-A/Followers")
            .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
            .header("X-CSRF-Token", self.csrf_token.to_owned())
            .query(&q)
            .build()
            .unwrap();
        let text = self
            .client
            .execute(req)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let res: FollowersResp = serde_json::from_str(&text).unwrap();
        return Ok(res);
    }

    async fn get_following(&self, uid: &String) -> Result<FollowingResp, Box<dyn Error>> {
        let mut variables = json!(
            {"userId":uid.as_str(),"count":20,"includePromotedContent":false}
        );
        let features = json!(
            {"responsive_web_graphql_exclude_directive_enabled":true,"verified_phone_label_enabled":false,"responsive_web_home_pinned_timelines_enabled":true,"creator_subscriptions_tweet_preview_api_enabled":true,"responsive_web_graphql_timeline_navigation_enabled":true,"responsive_web_graphql_skip_user_profile_image_extensions_enabled":false,"c9s_tweet_anatomy_moderator_badge_enabled":true,"tweetypie_unmention_optimization_enabled":true,"responsive_web_edit_tweet_api_enabled":true,"graphql_is_translatable_rweb_tweet_is_translatable_enabled":true,"view_counts_everywhere_api_enabled":true,"longform_notetweets_consumption_enabled":true,"responsive_web_twitter_article_tweet_consumption_enabled":false,"tweet_awards_web_tipping_enabled":false,"freedom_of_speech_not_reach_fetch_enabled":true,"standardized_nudges_misinfo":true,"tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled":true,"longform_notetweets_rich_text_read_enabled":true,"longform_notetweets_inline_media_enabled":true,"responsive_web_media_download_video_enabled":false,"responsive_web_enhance_cards_enabled":false}
        );
        variables["product"] = "Latest".into();
        let q = [
            ("variables", variables.to_string()),
            ("features", features.to_string()),
        ];
        let req = self
            .client
            .get("https://twitter.com/i/api/graphql/8cyc0OKedV_XD62fBjzxUw/Following")
            .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
            .header("X-CSRF-Token", self.csrf_token.to_owned())
            .query(&q)
            .build()
            .unwrap();
        let text = self
            .client
            .execute(req)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let res: FollowingResp = serde_json::from_str(&text).unwrap();
        return Ok(res);
    }
}

#[cfg(test)]
mod test_telation {
    use crate::{relation::Relation, ReAPI};

    async fn login(api: &mut ReAPI) -> Result<String, String> {
        dotenv::dotenv().ok();
        let name = std::env::var("TWITTER_USER_NAME").unwrap();
        let pwd = std::env::var("TWITTER_USER_PASSWORD").unwrap();
        api.login(&name, &pwd, "").await
    }

    #[tokio::test]
    async fn test_get_followers() {
        let uid = "1439140186378567683".to_string();
        let mut api = ReAPI::new();
        let _loggined = login(&mut api).await;
        let result = api.get_followers(&uid).await;
        println!("result {:?}", result);
    }

    #[tokio::test]
    async fn test_get_following() {
        let uid = "1439140186378567683".to_string();
        let mut api = ReAPI::new();
        let _loggined = login(&mut api).await;
        let result = api.get_following(&uid).await;
        println!("result {:?}", result);
    }
}
