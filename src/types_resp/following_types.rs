extern crate serde_json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FollowingResp {
    #[serde(rename = "data")]
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "user")]
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "result")]
    pub result: UserResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResult {
    #[serde(rename = "__typename")]
    pub typename: UserDisplayTypeEnum,

    #[serde(rename = "timeline")]
    pub timeline: ResultTimeline,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultTimeline {
    #[serde(rename = "timeline")]
    pub timeline: TimelineTimeline,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimelineTimeline {
    #[serde(rename = "instructions")]
    pub instructions: Vec<Instruction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Instruction {
    #[serde(rename = "type")]
    pub instruction_type: String,

    #[serde(rename = "direction")]
    pub direction: Option<String>,

    #[serde(rename = "entries")]
    pub entries: Option<Vec<Entry>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(rename = "entryId")]
    pub entry_id: String,

    #[serde(rename = "sortIndex")]
    pub sort_index: String,

    #[serde(rename = "content")]
    pub content: Content,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    #[serde(rename = "entryType")]
    pub entry_type: EntryTypeEnum,

    #[serde(rename = "__typename")]
    pub typename: EntryTypeEnum,

    #[serde(rename = "itemContent")]
    pub item_content: Option<ItemContent>,

    #[serde(rename = "clientEventInfo")]
    pub client_event_info: Option<ClientEventInfo>,

    #[serde(rename = "value")]
    pub value: Option<String>,

    #[serde(rename = "cursorType")]
    pub cursor_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientEventInfo {
    #[serde(rename = "component")]
    pub component: Component,

    #[serde(rename = "element")]
    pub element: Element,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemContent {
    #[serde(rename = "itemType")]
    pub item_type: ItemTypeEnum,

    #[serde(rename = "__typename")]
    pub typename: ItemTypeEnum,

    #[serde(rename = "user_results")]
    pub user_results: UserResults,

    #[serde(rename = "userDisplayType")]
    pub user_display_type: UserDisplayTypeEnum,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResults {
    #[serde(rename = "result")]
    pub result: UserResultsResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResultsResult {
    #[serde(rename = "__typename")]
    pub typename: UserDisplayTypeEnum,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "rest_id")]
    pub rest_id: String,

    #[serde(rename = "affiliates_highlighted_label")]
    pub affiliates_highlighted_label: AffiliatesHighlightedLabel,

    #[serde(rename = "has_graduated_access")]
    pub has_graduated_access: bool,

    #[serde(rename = "is_blue_verified")]
    pub is_blue_verified: bool,

    #[serde(rename = "profile_image_shape")]
    pub profile_image_shape: ProfileImageShape,

    #[serde(rename = "legacy")]
    pub legacy: Legacy,

    #[serde(rename = "professional")]
    pub professional: Option<Professional>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AffiliatesHighlightedLabel {
    #[serde(rename = "label")]
    pub label: Option<Label>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    #[serde(rename = "url")]
    pub url: LabelUrl,

    #[serde(rename = "badge")]
    pub badge: Badge,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "userLabelType")]
    pub user_label_type: String,

    #[serde(rename = "userLabelDisplayType")]
    pub user_label_display_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Badge {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LabelUrl {
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "urlType")]
    pub url_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Legacy {
    #[serde(rename = "can_dm")]
    pub can_dm: bool,

    #[serde(rename = "can_media_tag")]
    pub can_media_tag: bool,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "default_profile")]
    pub default_profile: bool,

    #[serde(rename = "default_profile_image")]
    pub default_profile_image: bool,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "entities")]
    pub entities: Entities,

    #[serde(rename = "fast_followers_count")]
    pub fast_followers_count: i64,

    #[serde(rename = "favourites_count")]
    pub favourites_count: i64,

    #[serde(rename = "followers_count")]
    pub followers_count: i64,

    #[serde(rename = "friends_count")]
    pub friends_count: i64,

    #[serde(rename = "has_custom_timelines")]
    pub has_custom_timelines: bool,

    #[serde(rename = "is_translator")]
    pub is_translator: bool,

    #[serde(rename = "listed_count")]
    pub listed_count: i64,

    #[serde(rename = "location")]
    pub location: String,

    #[serde(rename = "media_count")]
    pub media_count: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "normal_followers_count")]
    pub normal_followers_count: i64,

    #[serde(rename = "pinned_tweet_ids_str")]
    pub pinned_tweet_ids_str: Vec<String>,

    #[serde(rename = "possibly_sensitive")]
    pub possibly_sensitive: bool,

    #[serde(rename = "profile_banner_url")]
    pub profile_banner_url: Option<String>,

    #[serde(rename = "profile_image_url_https")]
    pub profile_image_url_https: String,

    #[serde(rename = "profile_interstitial_type")]
    pub profile_interstitial_type: String,

    #[serde(rename = "screen_name")]
    pub screen_name: String,

    #[serde(rename = "statuses_count")]
    pub statuses_count: i64,

    #[serde(rename = "translator_type")]
    pub translator_type: TranslatorType,

    #[serde(rename = "verified")]
    pub verified: bool,

    #[serde(rename = "want_retweets")]
    pub want_retweets: bool,

    #[serde(rename = "withheld_in_countries")]
    pub withheld_in_countries: Vec<Option<serde_json::Value>>,

    #[serde(rename = "url")]
    pub url: Option<String>,

    #[serde(rename = "following")]
    pub following: Option<bool>,

    #[serde(rename = "verified_type")]
    pub verified_type: Option<Type>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entities {
    #[serde(rename = "description")]
    pub description: Description,

    #[serde(rename = "url")]
    pub url: Option<Description>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    #[serde(rename = "urls")]
    pub urls: Vec<UrlElement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UrlElement {
    #[serde(rename = "display_url")]
    pub display_url: String,

    #[serde(rename = "expanded_url")]
    pub expanded_url: String,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "indices")]
    pub indices: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Professional {
    #[serde(rename = "rest_id")]
    pub rest_id: String,

    #[serde(rename = "professional_type")]
    pub professional_type: Type,

    #[serde(rename = "category")]
    pub category: Vec<Category>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "icon_name")]
    pub icon_name: IconName,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Component {
    #[serde(rename = "FollowingSgs")]
    FollowingSgs,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Element {
    #[serde(rename = "user")]
    User,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EntryTypeEnum {
    #[serde(rename = "TimelineTimelineCursor")]
    TimelineTimelineCursor,

    #[serde(rename = "TimelineTimelineItem")]
    TimelineTimelineItem,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ItemTypeEnum {
    #[serde(rename = "TimelineUser")]
    TimelineUser,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UserDisplayTypeEnum {
    #[serde(rename = "User")]
    User,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TranslatorType {
    #[serde(rename = "none")]
    None,

    #[serde(rename = "regular")]
    Regular,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
    #[serde(rename = "Business")]
    Business,

    #[serde(rename = "Creator")]
    Creator,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IconName {
    #[serde(rename = "IconBriefcaseStroke")]
    IconBriefcaseStroke,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProfileImageShape {
    #[serde(rename = "Circle")]
    Circle,

    #[serde(rename = "Square")]
    Square,

    #[serde(rename = "Hexagon")]
    Hexagon,
}
