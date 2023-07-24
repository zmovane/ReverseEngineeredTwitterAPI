use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ExpandedURL {
    pub expanded_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Url {
    pub urls: Vec<ExpandedURL>,
}

#[derive(Debug, Deserialize)]
pub struct EntitiesOfLegacyUser {
    pub url: Option<Url>,
}

#[derive(Debug, Deserialize)]
pub struct LegacyUser {
    pub created_at: String,
    pub description: String,
    pub entities: EntitiesOfLegacyUser,
    pub favourites_count: i128,
    pub followers_count: i128,
    pub friends_count: i128,
    pub id_str: Option<String>,
    pub listed_count: i128,
    pub name: String,
    pub location: String,
    pub pinned_tweet_ids_str: Vec<String>,
    pub profile_banner_url: String,
    pub profile_image_url_https: String,
    pub protected: Option<bool>,
    pub screen_name: String,
    pub statuses_count: i128,
    pub verified: bool,
}

#[derive(Debug, Deserialize)]
pub struct UserResult {
    pub is_blue_verified: bool,
    pub legacy: Option<LegacyUser>,
}

#[derive(Debug, Deserialize)]
pub struct UserResults {
    pub result: UserResult,
}

#[derive(Debug, Deserialize)]
pub struct ResultCore {
    pub user_results: UserResults,
}

#[derive(Debug, Deserialize)]
pub struct Views {
    pub count: String,
}

#[derive(Debug, Deserialize)]
pub struct NoteTweetResult {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct NoteTweetResults {
    pub result: NoteTweetResult,
}

#[derive(Debug, Deserialize)]
pub struct NoteTweet {
    pub note_tweet_results: NoteTweetResults,
}

#[derive(Debug, Deserialize)]
pub struct QuotedStatusResult {
    pub result: Box<Result>,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    pub __typename: String,
    pub core: Option<ResultCore>,
    pub views: Views,
    pub note_tweet: Option<NoteTweet>,
    pub quoted_status_result: Option<QuotedStatusResult>,
    pub legacy: LegacyTweet,
}

#[derive(Debug, Deserialize)]
pub struct TweetRresults {
    pub result: Result,
}

#[derive(Debug, Deserialize)]
pub struct ItemContent {
    #[serde(rename(serialize = "tweetDisplayType", deserialize = "tweetDisplayType"))]
    pub tweet_display_type: String,
    pub tweet_results: TweetRresults,
    #[serde(rename(serialize = "userDisplayType", deserialize = "userDisplayType"))]
    pub user_display_yype: Option<String>,
    pub user_results: Option<TweetRresults>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(rename(serialize = "itemContent", deserialize = "itemContent"))]
    pub item_content: ItemContent,
}

#[derive(Debug, Deserialize)]
pub struct EntryContent {
    #[serde(rename(serialize = "cursorType", deserialize = "cursorType"))]
    pub cursor_type: Option<String>,
    pub value: Option<String>,
    pub items: Option<Vec<Item>>,
    #[serde(rename(serialize = "itemContent", deserialize = "itemContent"))]
    pub item_content: Option<ItemContent>,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub content: EntryContent,
}

#[derive(Debug, Deserialize)]
pub struct Instruction {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub instruction_type: String,
    pub entries: Vec<Entry>,
    pub entry: Option<Entry>,
}

#[derive(Debug, Deserialize)]
pub struct Timeline {
    pub instructions: Option<Vec<Instruction>>,
}

#[derive(Debug, Deserialize)]
pub struct TimelineResult {
    pub timeline: Timeline,
}

#[derive(Debug, Deserialize)]
pub struct SearchTimeline {
    pub search_timeline: TimelineResult,
}

#[derive(Debug, Deserialize)]
pub struct SearchByRawQuery {
    pub search_by_raw_query: SearchTimeline,
}

#[derive(Debug, Deserialize)]
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
    pub url: Option<String>,
}

// Video type.
#[derive(Debug, Deserialize)]
pub struct Video {
    pub id: String,
    pub preview: String,
    pub url: Option<String>,
}

// GIF type.
#[derive(Debug, Deserialize)]
pub struct GIF {
    pub id: String,
    pub preview: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BoundingBox {
    pub _type: String,
    pub coordinates: Vec<Vec<Vec<f64>>>,
}

#[derive(Debug, Clone, Deserialize)]
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
    pub in_reply_to_status_id: Option<String>,
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
    pub place: Option<Place>,
    pub quoted_status: Option<Box<Tweet>>,
    pub quoted_status_id: Option<String>,
    pub replies: i128,
    pub retweets: i128,
    pub retweeted_status: Option<Box<Tweet>>,
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

#[derive(Debug, Deserialize)]
pub struct HashTag {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct TweetMedia {
    pub id_str: String,
    pub media_url_https: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub media_type: String,
    pub url: Option<String>,
    pub ext_sensitive_media_warning: Option<ExtSensitiveMediaWarning>,
    pub video_info: Option<VideoInfo>,
}

#[derive(Debug, Deserialize)]
pub struct Urls {
    pub expanded_url: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserMentions {
    pub id_str: String,
    pub name: String,
    pub screen_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Entities {
    pub hashtags: Vec<HashTag>,
    pub media: Option<Vec<TweetMedia>>,
    pub urls: Vec<Urls>,
    pub user_mentions: Vec<UserMentions>,
}

#[derive(Debug, Deserialize)]
pub struct ExtSensitiveMediaWarning {
    pub adult_content: bool,
    pub graphic_violence: bool,
    pub other: bool,
}

#[derive(Debug, Deserialize)]
pub struct Variant {
    pub bitrate: Option<i64>,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct VideoInfo {
    pub variants: Vec<Variant>,
}

#[derive(Debug, Deserialize)]
pub struct ExtendedMedia {
    pub id_str: String,
    pub media_url_https: String,
    pub ext_sensitive_media_warning: Option<ExtSensitiveMediaWarning>,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub ext_type: String,
    pub url: Option<String>,
    pub video_info: VideoInfo,
}

#[derive(Debug, Deserialize)]
pub struct ExtendedEntities {
    pub media: Vec<ExtendedMedia>,
}

#[derive(Debug, Deserialize)]
pub struct RetweetedStatusResult {
    pub result: Option<Box<Result>>,
}

#[derive(Debug, Deserialize)]
pub struct SelfThread {
    pub id_str: String,
}

#[derive(Debug, Deserialize)]
pub struct ExtViews {
    pub state: String,
    pub count: String,
}

#[derive(Debug, Deserialize)]
pub struct LegacyTweet {
    pub conversation_id_str: String,
    pub created_at: String,
    pub favorite_count: i128,
    pub full_text: String,
    pub entities: Entities,
    pub extended_entities: Option<ExtendedEntities>,
    pub id_str: String,
    pub in_reply_to_status_id_str: Option<String>,
    pub place: Option<Place>,
    pub reply_count: i128,
    pub retweet_count: i128,
    pub retweeted_status_id_str: Option<String>,
    pub retweeted_status_result: Option<RetweetedStatusResult>,
    pub quoted_status_id_str: Option<String>,
    pub self_thread: Option<SelfThread>,
    pub time: Option<String>,
    pub user_id_str: String,
    pub ext_views: Option<ExtViews>,
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
    let is_quoted = t
        .quoted_status_id_str
        .as_ref()
        .unwrap_or(&"".to_string())
        .ne("");
    let quoted_status_id = t.quoted_status_id_str.to_owned();
    let is_reply = t
        .in_reply_to_status_id_str
        .as_ref()
        .unwrap_or(&"".to_string())
        .ne("");
    let in_reply_to_status_id = t.in_reply_to_status_id_str.to_owned();
    let is_retweet = (t.in_reply_to_status_id_str.is_some()
        && t.in_reply_to_status_id_str
            .as_ref()
            .unwrap_or(&"".to_string())
            .ne(""))
        || (t.retweeted_status_result.is_some()
            && t.retweeted_status_result.as_ref().unwrap().result.is_some());
    let retweeted_status_id = t
        .retweeted_status_id_str
        .as_ref()
        .unwrap_or(&String::from(""))
        .to_string();
    let mut views = 0i128;

    if t.ext_views.is_some() {
        views = t.ext_views.as_ref().unwrap().count.parse::<i128>().unwrap();
    }

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
    for i in t.entities.media.as_ref().unwrap_or(&vec![]).iter() {
        match i.media_type.as_str() {
            "photo" => photos.push(Photo {
                id: i.id_str.to_owned(),
                url: Some(i.media_url_https.to_owned()),
            }),
            "animated_gif" | "video" => {
                let mut url = "";
                let mut max_bitrate = 0;
                if i.video_info.is_some() {
                    let video_info = i.video_info.as_ref().unwrap();
                    for variant in &video_info.variants {
                        let bitrate = variant.bitrate.unwrap_or(0);
                        if bitrate > max_bitrate {
                            max_bitrate = bitrate;
                            url = variant.url.strip_suffix("?tag=10").unwrap();
                        }
                    }
                }
                if i.media_type.as_str().eq("video") {
                    videos.push(Video {
                        id: i.id_str.to_owned(),
                        preview: i.media_url_https.to_owned(),
                        url: Some(url.to_string()),
                    })
                } else {
                    gifs.push(GIF {
                        id: i.id_str.to_owned(),
                        preview: i.media_url_https.to_owned(),
                        url: Some(url.to_string()),
                    })
                }
            }
            _ => {}
        }
        if !sensitive_content && i.ext_sensitive_media_warning.is_some() {
            let warning = i.ext_sensitive_media_warning.as_ref().unwrap();
            sensitive_content = warning.adult_content || warning.graphic_violence || warning.other;
        }
    }
    let mut urls: Vec<String> = vec![];
    for i in t.entities.urls.iter() {
        urls.push(i.expanded_url.to_owned());
    }

    let mut time_parsed = chrono::offset::Utc::now().fixed_offset();
    if t.time.is_some() {
        time_parsed = DateTime::parse_from_rfc2822(&t.time.as_ref().unwrap()).unwrap();
    }
    let timestamp = time_parsed.timestamp();
    let html = t.full_text.to_owned();

    let mut retweeted_status: Option<Box<Tweet>> = Option::None;
    if t.retweeted_status_result.is_some() {
        let core = &t
            .retweeted_status_result
            .as_ref()
            .unwrap()
            .result
            .as_ref()
            .unwrap()
            .core;
        if let Some(core) = core {
            let legacy_u = core.user_results.result.legacy.as_ref().unwrap();
            let legacy_t = &t
                .retweeted_status_result
                .as_ref()
                .unwrap()
                .result
                .as_ref()
                .unwrap()
                .legacy;
            retweeted_status = Some(Box::new(parse_legacy_tweet(&legacy_u, &legacy_t).unwrap()));
        }
    }
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
