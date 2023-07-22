use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExpandedURL {
    expanded_url: String,
}

#[derive(Deserialize)]
pub struct Url {
    urls: Vec<ExpandedURL>,
}

#[derive(Deserialize)]
pub struct EntitiesOfLegacyUser {
    url: Url,
}

#[derive(Deserialize)]
pub struct LegacyUser {
    created_at: String,
    description: String,
    entities: EntitiesOfLegacyUser,
}

#[derive(Deserialize)]
pub struct UserResult {
    is_blue_verified: bool,
    legacy: Option<LegacyUser>,
}

#[derive(Deserialize)]
pub struct UserResults {
    result: UserResult,
}

#[derive(Deserialize)]
pub struct ResultCore {
    user_results: UserResults,
}

#[derive(Deserialize)]
pub struct Views {
    count: String,
}

#[derive(Deserialize)]
pub struct NoteTweetResult {
    text: String,
}

#[derive(Deserialize)]
pub struct NoteTweetResults {
    result: NoteTweetResult,
}

#[derive(Deserialize)]
pub struct NoteTweet {
    note_tweet_results: NoteTweetResults,
}

#[derive(Deserialize)]
pub struct QuotedStatusResult {
    result: Box<Result>,
}

#[derive(Deserialize)]
pub struct Result {
    __typename: String,
    core: ResultCore,
    views: Views,
    note_tweet: NoteTweet,
    quoted_status_result: QuotedStatusResult,
    legacy: LegacyTweet,
}

#[derive(Deserialize)]
pub struct TweetRresults {
    result: Result,
}

#[derive(Deserialize)]
pub struct ItemContent {
    #[serde(rename(serialize = "tweetDisplayType", deserialize = "tweetDisplayType"))]
    tweet_display_type: String,
    tweet_results: TweetRresults,
    #[serde(rename(serialize = "userDisplayType", deserialize = "userDisplayType"))]
    user_display_yype: Option<String>,
    user_results: Option<TweetRresults>,
}

#[derive(Deserialize)]
pub struct Item {
    #[serde(rename(serialize = "itemContent", deserialize = "itemContent"))]
    item_content: ItemContent,
}

#[derive(Deserialize)]
pub struct EntryContent {
    #[serde(rename(serialize = "cursorType", deserialize = "cursorType"))]
    cursor_type: String,
    value: String,
    items: Vec<Item>,
    item_content: ItemContent,
}

#[derive(Deserialize)]
pub struct Entry {
    content: EntryContent,
}

#[derive(Deserialize)]
pub struct Instruction {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    instruction_type: String,
    entries: Vec<Entry>,
    entry: Option<Entry>,
}

#[derive(Deserialize)]
pub struct Timeline {
    instructions: Option<Vec<Instruction>>,
}

#[derive(Deserialize)]
pub struct TimelineResult {
    timeline: Timeline,
}

#[derive(Deserialize)]
pub struct SearchTimeline {
    search_timeline: Timeline,
}

#[derive(Deserialize)]
pub struct SearchByRawQuery {
    search_by_raw_query: SearchTimeline,
}

#[derive(Deserialize)]
pub struct Data {
    data: SearchByRawQuery,
}

// Mention type.
pub struct Mention {
    id: String,
    username: String,
    name: String,
}

// Photo type.
pub struct Photo {
    id: String,
    url: String,
}

// Video type.
pub struct Video {
    id: String,
    preview: String,
    url: String,
}

// GIF type.
pub struct GIF {
    id: String,
    preview: String,
    url: String,
}

#[derive(Deserialize)]
pub struct BoundingBox {
    _type: String,
    coordinates: Vec<Vec<Vec<f64>>>,
}

#[derive(Deserialize)]
pub struct Place {
    id: String,
    place_type: String,
    name: String,
    full_name: String,
    country_code: String,
    country: String,
    bounding_box: BoundingBox,
}

pub struct Tweet {
    converation_id: String,
    gifs: Vec<GIF>,
    hash_tags: Vec<String>,
    html: Vec<String>,
    id: String,
    in_reply_to_status: Box<Tweet>,
    in_reply_to_status_id: String,
    is_quoted: bool,
    is_pin: bool,
    is_reply: bool,
    is_retweet: bool,
    is_self_thread: bool,
    likes: i64,
    name: String,
    mentions: Vec<Mention>,
    permanent_url: String,
    photos: Vec<Photo>,
    place: Place,
    quoted_status: Box<Tweet>,
    quoted_status_id: String,
    replies: i64,
    retweets: i64,
    retweeted_status: Box<Tweet>,
    retweeted_status_id: String,
    text: String,
    thread: Vec<Box<Tweet>>,
    time_parsed: DateTime<Utc>,
    timestamp: i64,
    urls: Vec<String>,
    user_id: String,
    username: String,
    videos: Vec<Video>,
    views: i64,
    sensitive_content: bool,
}

#[derive(Deserialize)]
pub struct HashTag {
    text: String,
}

#[derive(Deserialize)]
pub struct Media {
    media_url_https: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    media_type: String,
    url: String,
}

#[derive(Deserialize)]
pub struct Urls {
    expanded_url: String,
    url: String,
}

#[derive(Deserialize)]
pub struct UserMentions {
    id_str: String,
    name: String,
    screen_name: String,
}

#[derive(Deserialize)]
pub struct Entities {
    hashtags: Vec<HashTag>,
    media: Vec<Media>,
    urls: Vec<Urls>,
    user_mentions: Vec<UserMentions>,
}

#[derive(Deserialize)]
pub struct ExtSensitiveMediaWarning {
    adult_content: bool,
    graphic_violence: bool,
    other: bool,
}

#[derive(Deserialize)]
pub struct Variant {
    bitrate: i64,
    url: String,
}

#[derive(Deserialize)]
pub struct VideoInfo {
    variants: Vec<Variant>,
}

#[derive(Deserialize)]
pub struct ExtendedMedia {
    id_str: String,
    media_url_https: String,
    ext_sensitive_media_warning: ExtSensitiveMediaWarning,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    ext_type: String,
    url: String,
    video_info: VideoInfo,
}

#[derive(Deserialize)]
pub struct ExtendedEntities {
    media: ExtendedMedia,
}

#[derive(Deserialize)]
pub struct RetweetedStatusResult {
    result: Box<Result>,
}

#[derive(Deserialize)]
pub struct SelfThread {
    id_str: String,
}

#[derive(Deserialize)]
pub struct ExtViews {
    state: String,
    count: String,
}

#[derive(Deserialize)]
pub struct LegacyTweet {
    conversation_id_str: String,
    created_at: String,
    favorite_count: u64,
    full_text: String,
    entities: Entities,
    extended_entities: ExtendedEntities,
    id_str: String,
    in_reply_to_status_id_str: String,
    place: Place,
    reply_count: i128,
    retweet_count: i128,
    retweeted_status_id_str: String,
    retweeted_status_result: RetweetedStatusResult,
    quoted_status_id_str: String,
    self_thread: SelfThread,
    time: String,
    user_id_str: String,
    ext_views: ExtViews,
}
