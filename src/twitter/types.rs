use chrono::{DateTime, FixedOffset, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExpandedURL {
    pub expanded_url: String,
}

#[derive(Deserialize)]
pub struct Url {
    pub urls: Vec<ExpandedURL>,
}

#[derive(Deserialize)]
pub struct EntitiesOfLegacyUser {
    pub url: Url,
}

#[derive(Deserialize)]
pub struct LegacyUser {
    pub created_at: String,
    pub description: String,
    pub entities: EntitiesOfLegacyUser,
    pub favourites_count: i128,
    pub followers_count: i128,
    pub friends_count: i128,
    pub id_str: String,
    pub listed_count: i128,
    pub name: String,
    pub location: String,
    pub pinned_tweet_ids_str: Vec<String>,
    pub profile_banner_url: String,
    pub profile_image_url_https: String,
    pub protected: bool,
    pub screen_name: String,
    pub statuses_count: i128,
    pub verified: bool,
}

#[derive(Deserialize)]
pub struct UserResult {
    pub is_blue_verified: bool,
    pub legacy: Option<LegacyUser>,
}

#[derive(Deserialize)]
pub struct UserResults {
    pub result: UserResult,
}

#[derive(Deserialize)]
pub struct ResultCore {
    pub user_results: UserResults,
}

#[derive(Deserialize)]
pub struct Views {
    pub count: String,
}

#[derive(Deserialize)]
pub struct NoteTweetResult {
    pub text: String,
}

#[derive(Deserialize)]
pub struct NoteTweetResults {
    pub result: NoteTweetResult,
}

#[derive(Deserialize)]
pub struct NoteTweet {
    pub note_tweet_results: NoteTweetResults,
}

#[derive(Deserialize)]
pub struct QuotedStatusResult {
    pub result: Box<Result>,
}

#[derive(Deserialize)]
pub struct Result {
    pub __typename: String,
    pub core: ResultCore,
    pub views: Views,
    pub note_tweet: NoteTweet,
    pub quoted_status_result: QuotedStatusResult,
    pub legacy: LegacyTweet,
}

#[derive(Deserialize)]
pub struct TweetRresults {
    pub result: Result,
}

#[derive(Deserialize)]
pub struct ItemContent {
    #[serde(rename(serialize = "tweetDisplayType", deserialize = "tweetDisplayType"))]
    pub tweet_display_type: String,
    pub tweet_results: TweetRresults,
    #[serde(rename(serialize = "userDisplayType", deserialize = "userDisplayType"))]
    pub user_display_yype: Option<String>,
    pub user_results: Option<TweetRresults>,
}

#[derive(Deserialize)]
pub struct Item {
    #[serde(rename(serialize = "itemContent", deserialize = "itemContent"))]
    pub item_content: ItemContent,
}

#[derive(Deserialize)]
pub struct EntryContent {
    #[serde(rename(serialize = "cursorType", deserialize = "cursorType"))]
    pub cursor_type: Option<String>,
    pub value: Option<String>,
    pub items: Option<Vec<Item>>,
    pub item_content: Option<ItemContent>,
}

#[derive(Deserialize)]
pub struct Entry {
    pub content: EntryContent,
}

#[derive(Deserialize)]
pub struct Instruction {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub instruction_type: String,
    pub entries: Vec<Entry>,
    pub entry: Option<Entry>,
}

#[derive(Deserialize)]
pub struct Timeline {
    pub instructions: Option<Vec<Instruction>>,
}

#[derive(Deserialize)]
pub struct TimelineResult {
    pub timeline: Timeline,
}

#[derive(Deserialize)]
pub struct SearchTimeline {
    pub search_timeline: TimelineResult,
}

#[derive(Deserialize)]
pub struct SearchByRawQuery {
    pub search_by_raw_query: SearchTimeline,
}

#[derive(Deserialize)]
pub struct Data {
    pub data: SearchByRawQuery,
}

// Mention type.
#[derive(Debug, Deserialize)]
pub struct Mention {
    pub id: String,
    pub username: String,
    pub name: String,
}

// Photo type.
#[derive(Debug, Deserialize)]
pub struct Photo {
    pub id: String,
    pub url: String,
}

// Video type.
#[derive(Debug, Deserialize)]
pub struct Video {
    pub id: String,
    pub preview: String,
    pub url: String,
}

// GIF type.
#[derive(Debug, Deserialize)]
pub struct GIF {
    pub id: String,
    pub preview: String,
    pub url: String,
}

#[derive(Clone, Deserialize)]
pub struct BoundingBox {
    pub _type: String,
    pub coordinates: Vec<Vec<Vec<f64>>>,
}

#[derive(Clone, Deserialize)]
pub struct Place {
    pub id: String,
    pub place_type: String,
    pub name: String,
    pub full_name: String,
    pub country_code: String,
    pub country: String,
    pub bounding_box: BoundingBox,
}

pub struct Tweet {
    pub converation_id: String,
    pub gifs: Vec<GIF>,
    pub hash_tags: Vec<String>,
    pub html: String,
    pub id: String,
    pub in_reply_to_status: Option<Box<Tweet>>,
    pub in_reply_to_status_id: String,
    pub is_quoted: bool,
    pub is_pin: bool,
    pub is_reply: bool,
    pub is_retweet: bool,
    pub is_self_thread: bool,
    pub likes: i128,
    pub name: String,
    pub mentions: Vec<Mention>,
    pub permanent_url: String,
    pub photos: Vec<Photo>,
    pub place: Place,
    pub quoted_status: Option<Box<Tweet>>,
    pub quoted_status_id: String,
    pub replies: i128,
    pub retweets: i128,
    pub retweeted_status: Box<Tweet>,
    pub retweeted_status_id: String,
    pub text: String,
    pub thread: Vec<Box<Tweet>>,
    pub time_parsed: DateTime<FixedOffset>,
    pub timestamp: i64,
    pub urls: Vec<String>,
    pub user_id: String,
    pub username: String,
    pub videos: Vec<Video>,
    pub views: i128,
    pub sensitive_content: bool,
}

#[derive(Deserialize)]
pub struct HashTag {
    pub text: String,
}

#[derive(Deserialize)]
pub struct TweetMedia {
    pub id_str: String,
    pub media_url_https: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub media_type: String,
    pub url: String,
    pub ext_sensitive_media_warning: ExtSensitiveMediaWarning,
    pub video_info: VideoInfo,
}

#[derive(Deserialize)]
pub struct Urls {
    pub expanded_url: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct UserMentions {
    pub id_str: String,
    pub name: String,
    pub screen_name: String,
}

#[derive(Deserialize)]
pub struct Entities {
    pub hashtags: Vec<HashTag>,
    pub media: Vec<TweetMedia>,
    pub urls: Vec<Urls>,
    pub user_mentions: Vec<UserMentions>,
}

#[derive(Deserialize)]
pub struct ExtSensitiveMediaWarning {
    pub adult_content: bool,
    pub graphic_violence: bool,
    pub other: bool,
}

#[derive(Deserialize)]
pub struct Variant {
    pub bitrate: i64,
    pub url: String,
}

#[derive(Deserialize)]
pub struct VideoInfo {
    pub variants: Vec<Variant>,
}

#[derive(Deserialize)]
pub struct ExtendedMedia {
    pub id_str: String,
    pub media_url_https: String,
    pub ext_sensitive_media_warning: ExtSensitiveMediaWarning,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub ext_type: String,
    pub url: String,
    pub video_info: VideoInfo,
}

#[derive(Deserialize)]
pub struct ExtendedEntities {
    pub media: ExtendedMedia,
}

#[derive(Deserialize)]
pub struct RetweetedStatusResult {
    pub result: Option<Box<Result>>,
}

#[derive(Deserialize)]
pub struct SelfThread {
    id_str: String,
}

#[derive(Deserialize)]
pub struct ExtViews {
    pub state: String,
    pub count: String,
}

#[derive(Deserialize)]
pub struct LegacyTweet {
    pub conversation_id_str: String,
    pub created_at: String,
    pub favorite_count: i128,
    pub full_text: String,
    pub entities: Entities,
    pub extended_entities: ExtendedEntities,
    pub id_str: String,
    pub in_reply_to_status_id_str: String,
    pub place: Place,
    pub reply_count: i128,
    pub retweet_count: i128,
    pub retweeted_status_id_str: String,
    pub retweeted_status_result: RetweetedStatusResult,
    pub quoted_status_id_str: String,
    pub self_thread: SelfThread,
    pub time: String,
    pub user_id_str: String,
    pub ext_views: ExtViews,
}

pub fn parse_legacy_tweet(u: &LegacyUser, t: &LegacyTweet) -> Option<Tweet> {
    let tweet_id = &t.id_str;
    if tweet_id.eq("") {
        return Option::None;
    }
    let id = t.id_str.to_owned();
    let name = u.name.to_owned();
    let likes = t.favorite_count;
    let user_id = t.user_id_str.to_owned();
    let username = u.screen_name.to_owned();
    let converation_id = t.conversation_id_str.to_owned();
    let permanent_url = format!("https://twitter.com/{}/status/{}", username, tweet_id);
    let replies = t.reply_count;
    let retweets = t.retweet_count;
    let text = t.full_text.to_owned();
    let is_quoted = t.quoted_status_id_str.ne("");
    let quoted_status_id = t.quoted_status_id_str.to_owned();
    let is_reply = t.in_reply_to_status_id_str.ne("");
    let in_reply_to_status_id = t.in_reply_to_status_id_str.to_owned();
    let is_retweet =
        t.in_reply_to_status_id_str.ne("") || t.retweeted_status_result.result.is_some();
    let retweeted_status_id = t.retweeted_status_id_str.to_owned();
    let views = t.ext_views.count.parse::<i128>().unwrap();
    let hash_tags: Vec<String> = t
        .entities
        .hashtags
        .iter()
        .map(|i| i.text.to_owned())
        .collect();
    let mentions: Vec<Mention> = t
        .entities
        .user_mentions
        .iter()
        .map(|i| Mention {
            id: i.id_str.to_owned(),
            username: i.screen_name.to_owned(),
            name: i.name.to_owned(),
        })
        .collect();
    let mut photos: Vec<Photo> = vec![];
    let mut videos: Vec<Video> = vec![];
    let mut gifs: Vec<GIF> = vec![];
    let mut sensitive_content = false;
    for i in t.entities.media.iter() {
        match i.media_type.as_str() {
            "photo" => photos.push(Photo {
                id: i.id_str.to_owned(),
                url: i.media_url_https.to_owned(),
            }),
            "animated_gif" | "video" => {
                let mut url = "";
                let mut max_bitrate = 0;
                for variant in &i.video_info.variants {
                    if variant.bitrate > max_bitrate {
                        max_bitrate = variant.bitrate;
                        url = variant.url.strip_suffix("?tag=10").unwrap();
                    }
                }
                if i.media_type.as_str().eq("video") {
                    videos.push(Video {
                        id: i.id_str.to_owned(),
                        preview: i.media_url_https.to_owned(),
                        url: url.to_string(),
                    })
                } else {
                    gifs.push(GIF {
                        id: i.id_str.to_owned(),
                        preview: i.media_url_https.to_owned(),
                        url: url.to_string(),
                    })
                }
            }
            _ => {}
        }
        if !sensitive_content {
            let warning = &i.ext_sensitive_media_warning;
            sensitive_content = warning.adult_content || warning.graphic_violence || warning.other;
        }
    }
    let mut urls: Vec<String> = vec![];
    for i in t.entities.urls.iter() {
        urls.push(i.expanded_url.to_owned());
    }
    let time_parsed = DateTime::parse_from_rfc2822(&t.time).unwrap();
    let timestamp = time_parsed.timestamp();
    let html = t.full_text.to_owned();

    let legacy_u = t
        .retweeted_status_result
        .result
        .as_ref()
        .unwrap()
        .core
        .user_results
        .result
        .legacy
        .as_ref()
        .unwrap();
    let legacy_t = &t.retweeted_status_result.result.as_ref().unwrap().legacy;
    let retweeted_status = Box::new(parse_legacy_tweet(&legacy_u, &legacy_t).unwrap());
    let tweet = Tweet {
        converation_id,
        id,
        likes,
        name,
        permanent_url,
        replies,
        retweets,
        text,
        user_id,
        username,
        place: t.place.clone(),
        is_pin: false,
        is_self_thread: false,
        thread: vec![],
        is_quoted,
        quoted_status_id,
        quoted_status: Option::None,
        is_reply,
        in_reply_to_status_id,
        in_reply_to_status: Option::None,
        is_retweet,
        retweeted_status_id,
        retweeted_status,
        views,
        hash_tags,
        mentions,
        gifs,
        videos,
        photos,
        urls,
        time_parsed,
        timestamp,
        sensitive_content,
        html,
    };
    Some(tweet)
}
