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
    async fn get_following(
        &self,
        uid: &String,
        cursor: Option<String>,
    ) -> Result<FollowingResp, Box<dyn Error>>;
    async fn check_following(
        &self,
        uid: &String,
        target_uid: &String,
    ) -> Result<bool, Box<dyn Error>>;
}

#[async_trait]
impl Relation for ReAPI {
    async fn check_following(
        &self,
        uid: &String,
        target_uid: &String,
    ) -> Result<bool, Box<dyn Error>> {
        // check whether target_uid is in following list
        let mut is_following = false;
        let mut cursor: Option<String> = None;
        let mut is_continue = true;
        while is_continue {
            // if cursor is not empty, then use cursor
            let res: FollowingResp = self.get_following(uid, cursor.clone()).await?;
            res.data
                .user
                .result
                .timeline
                .timeline
                .instructions
                .iter()
                .for_each(|instruction| {
                    if let Some(entries) = &instruction.entries {
                        entries.iter().for_each(|entry| {
                            // if target_uid is == user-xxxxxxx
                            let target_uid_str = format!("user-{}", target_uid);
                            if entry.entry_id == target_uid_str {
                                is_following = true;
                                is_continue = false;
                            }
                            // cursor Type exists
                            let cursor_type =
                                &entry.content.cursor_type.clone().unwrap_or_default();
                            let value = &entry.content.value.clone().unwrap_or_default();
                            // check whether cursor_type is "Bottom" and the content value is not strat with 0
                            if cursor_type == "Bottom" && !value.starts_with("0") {
                                cursor = Some(value.to_string());
                                is_continue = true;
                            } else {
                                is_continue = false;
                                is_following = false;
                            }
                        })
                    }
                });
            // sleep 0.5s
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
        Ok(is_following)
    }

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

    async fn get_following(
        &self,
        uid: &String,
        cursor: Option<String>,
    ) -> Result<FollowingResp, Box<dyn Error>> {
        let mut variables = json!(
            {"userId":uid.as_str(),"count":20,"includePromotedContent":false}
        );
        let features = json!(
            {"responsive_web_graphql_exclude_directive_enabled":true,"verified_phone_label_enabled":false,"responsive_web_home_pinned_timelines_enabled":true,"creator_subscriptions_tweet_preview_api_enabled":true,"responsive_web_graphql_timeline_navigation_enabled":true,"responsive_web_graphql_skip_user_profile_image_extensions_enabled":false,"c9s_tweet_anatomy_moderator_badge_enabled":true,"tweetypie_unmention_optimization_enabled":true,"responsive_web_edit_tweet_api_enabled":true,"graphql_is_translatable_rweb_tweet_is_translatable_enabled":true,"view_counts_everywhere_api_enabled":true,"longform_notetweets_consumption_enabled":true,"responsive_web_twitter_article_tweet_consumption_enabled":false,"tweet_awards_web_tipping_enabled":false,"freedom_of_speech_not_reach_fetch_enabled":true,"standardized_nudges_misinfo":true,"tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled":true,"longform_notetweets_rich_text_read_enabled":true,"longform_notetweets_inline_media_enabled":true,"responsive_web_media_download_video_enabled":false,"responsive_web_enhance_cards_enabled":false}
        );
        variables["product"] = "Latest".into();
        // check cursor
        if let Some(c) = cursor {
            variables["cursor"] = c.as_str().into();
        }
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
        let res = serde_json::from_str(&text);
        if let Err(e) = res {
            return Err(Box::new(e));
        }
        return Ok(res.unwrap());
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
        let result = api.get_following(&uid, None).await;
        println!("result {:?}", result);
    }

    #[tokio::test]
    async fn test_user_follow_target_user() {
        let uid = "1439140186378567683".to_string();
        let target_uid = "1456507428208398336".to_string();
        let mut api = ReAPI::new();
        let _loggined = login(&mut api).await;
        let result = api.check_following(&uid, &target_uid).await;
        println!("user is following {:?}", result);
    }
}
