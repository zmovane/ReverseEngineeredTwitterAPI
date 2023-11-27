extern crate serde_json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FollowersResp {
    #[serde(rename = "data")]
    data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "user")]
    user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "result")]
    result: UserResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResult {
    #[serde(rename = "__typename")]
    typename: String,

    #[serde(rename = "timeline")]
    timeline: ResultTimeline,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultTimeline {
    #[serde(rename = "timeline")]
    timeline: TimelineTimeline,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimelineTimeline {
    #[serde(rename = "instructions")]
    instructions: Vec<Instruction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Instruction {
    #[serde(rename = "type")]
    instruction_type: String,

    #[serde(rename = "direction")]
    direction: Option<String>,

    #[serde(rename = "entries")]
    entries: Option<Vec<Entry>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(rename = "entryId")]
    entry_id: String,

    #[serde(rename = "sortIndex")]
    sort_index: String,

    #[serde(rename = "content")]
    content: Content,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    #[serde(rename = "entryType")]
    entry_type: String,

    #[serde(rename = "__typename")]
    typename: String,

    #[serde(rename = "itemContent")]
    item_content: Option<ItemContent>,

    #[serde(rename = "clientEventInfo")]
    client_event_info: Option<ClientEventInfo>,

    #[serde(rename = "value")]
    value: Option<String>,

    #[serde(rename = "cursorType")]
    cursor_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientEventInfo {
    #[serde(rename = "component")]
    component: String,

    #[serde(rename = "element")]
    element: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemContent {
    #[serde(rename = "itemType")]
    item_type: String,

    #[serde(rename = "__typename")]
    typename: String,

    #[serde(rename = "user_results")]
    user_results: UserResults,

    #[serde(rename = "userDisplayType")]
    user_display_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResults {
    #[serde(rename = "result")]
    result: UserResultsResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResultsResult {
    #[serde(rename = "__typename")]
    typename: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "rest_id")]
    rest_id: String,

    #[serde(rename = "affiliates_highlighted_label")]
    affiliates_highlighted_label: AffiliatesHighlightedLabel,

    #[serde(rename = "has_graduated_access")]
    has_graduated_access: bool,

    #[serde(rename = "is_blue_verified")]
    is_blue_verified: bool,

    #[serde(rename = "profile_image_shape")]
    profile_image_shape: String,

    #[serde(rename = "legacy")]
    legacy: Legacy,

    #[serde(rename = "professional")]
    professional: Option<Professional>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AffiliatesHighlightedLabel {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Legacy {
    #[serde(rename = "followed_by")]
    followed_by: Option<bool>,

    #[serde(rename = "can_dm")]
    can_dm: bool,

    #[serde(rename = "can_media_tag")]
    can_media_tag: bool,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "default_profile")]
    default_profile: bool,

    #[serde(rename = "default_profile_image")]
    default_profile_image: bool,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "entities")]
    entities: Entities,

    #[serde(rename = "fast_followers_count")]
    fast_followers_count: i64,

    #[serde(rename = "favourites_count")]
    favourites_count: i64,

    #[serde(rename = "followers_count")]
    followers_count: i64,

    #[serde(rename = "friends_count")]
    friends_count: i64,

    #[serde(rename = "has_custom_timelines")]
    has_custom_timelines: bool,

    #[serde(rename = "is_translator")]
    is_translator: bool,

    #[serde(rename = "listed_count")]
    listed_count: i64,

    #[serde(rename = "location")]
    location: String,

    #[serde(rename = "media_count")]
    media_count: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "normal_followers_count")]
    normal_followers_count: i64,

    #[serde(rename = "pinned_tweet_ids_str")]
    pinned_tweet_ids_str: Vec<String>,

    #[serde(rename = "possibly_sensitive")]
    possibly_sensitive: bool,

    #[serde(rename = "profile_banner_url")]
    profile_banner_url: Option<String>,

    #[serde(rename = "profile_image_url_https")]
    profile_image_url_https: String,

    #[serde(rename = "profile_interstitial_type")]
    profile_interstitial_type: String,

    #[serde(rename = "screen_name")]
    screen_name: String,

    #[serde(rename = "statuses_count")]
    statuses_count: i64,

    #[serde(rename = "translator_type")]
    translator_type: String,

    #[serde(rename = "verified")]
    verified: bool,

    #[serde(rename = "want_retweets")]
    want_retweets: bool,

    #[serde(rename = "withheld_in_countries")]
    withheld_in_countries: Vec<Option<serde_json::Value>>,

    #[serde(rename = "following")]
    following: Option<bool>,

    #[serde(rename = "url")]
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entities {
    #[serde(rename = "description")]
    description: Description,

    #[serde(rename = "url")]
    url: Option<Description>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    #[serde(rename = "urls")]
    urls: Vec<Url>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Url {
    #[serde(rename = "display_url")]
    display_url: String,

    #[serde(rename = "expanded_url")]
    expanded_url: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "indices")]
    indices: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Professional {
    #[serde(rename = "rest_id")]
    rest_id: String,

    #[serde(rename = "professional_type")]
    professional_type: String,

    #[serde(rename = "category")]
    category: Vec<Option<serde_json::Value>>,
}
