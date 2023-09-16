use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeNickNameInfo {
    pub can_change: bool,
    pub next_change_need_days: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VipInfo {
    pub discount: i32,
    pub level: usize,
    pub next_discount: i32,
    pub next_discount_level: usize,
    pub next_discount_level_point: usize,
    pub next_level: usize,
    pub next_level_point: usize,
    pub point: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPrivateExpand {
    pub change_nick_name_info: Option<ChangeNickNameInfo>,
    pub has_active_unlock_chap_with_ad: Option<bool>,
    pub has_order_chap_with_fire_money: Option<bool>,
    pub has_ordered_vip_chaps: Option<bool>,
    pub has_paid_first_time: Option<bool>,
    pub has_unlock_chap_with_ad: Option<bool>,
    pub is_real_name_auth: Option<bool>,
    pub redpacket_code: Option<String>,
    pub use_welfaresys: Option<bool>,
    pub used_redpacket_code: Option<String>,
    pub vip_info: Option<VipInfo>,
    pub welfare_coin: Option<i32>,
    pub welfare_money: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPrivate {
    pub account_id: i32,
    pub avatar: String,
    pub country_code: u32,
    pub email: String,
    pub expand: Option<UserPrivateExpand>,
    pub fire_coin: i32,
    pub is_author: bool,
    pub nick_name: String,
    pub phone_num: String,
    pub register_date: String,
    pub role_name: String,
    pub user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserExpand {
    pub avatar: Option<String>,
    pub avatar_frame: Option<String>,
    pub background_pic: Option<String>,
    pub big_avatar: Option<String>,
    pub fans_num: Option<usize>,
    pub follow_num: Option<usize>,
    pub followyou: Option<bool>,
    pub introduction: Option<String>,
    pub verify_info: Option<String>,
    pub verify_type: Option<i32>,
    pub widgets: Option<serde_json::Value>,
    pub youblock: Option<bool>,
    pub youfollow: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub account_id: i32,
    pub expand: Option<UserExpand>,
    pub nick_name: String,
    pub user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemTag {
    pub sys_tag_id: i32,
    pub tag_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankingList {
    pub date_range: i32,
    pub desc: String,
    pub r#type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterRef {
    pub add_time: String,
    pub chap_id: i32,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NovelExpand {
    pub audit_cover: Option<String>,
    pub big_bg_banner: Option<String>,
    pub big_novel_cover: Option<String>,
    pub chapter_count: Option<usize>,
    pub custom_tag: Option<Vec<String>>,
    pub discount: Option<f32>,
    pub discount_expire_date: Option<String>,
    pub essay_tag: Option<String>,
    pub fav: Option<usize>,
    pub first_chapter: Option<ChapterRef>,
    pub home_flag: Option<Vec<String>>,
    pub intro: Option<String>,
    pub is_banch: Option<bool>,
    pub last_chapter: Option<ChapterRef>,
    pub latest_comment_date: Option<String>,
    pub origin_total_need_fire_money: Option<usize>,
    pub point_count: Option<usize>,
    pub pre_order_info: Option<String>,
    pub rankinglist: Option<RankingList>,
    pub sign_level: Option<String>,
    pub sys_tags: Option<Vec<SystemTag>>,
    pub tags: Option<Vec<String>>,
    pub ticket: Option<usize>,
    pub topic: Option<String>,
    pub total_need_fire_money: Option<usize>,
    pub type_name: Option<String>,
    pub unaudited_customtag: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Novel {
    pub add_time: String,
    pub allow_down: bool,
    pub author_id: i32,
    pub author_name: String,
    pub bg_banner: String,
    pub category_id: i32,
    pub char_count: usize,
    pub expand: Option<NovelExpand>,
    pub is_finish: bool,
    pub is_sensitive: bool,
    pub last_update_time: String,
    pub mark_count: usize,
    pub novel_cover: String,
    pub novel_id: Option<i32>,
    pub novel_name: String,
    pub point: f32,
    pub sign_status: String,
    pub type_id: usize,
    pub view_times: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterExpand {
    pub author_talk: Option<serde_json::Value>,
    pub chat_lines: Option<serde_json::Value>,
    pub content: Option<String>,
    pub is_branch: Option<bool>,
    pub is_content_encrypted: Option<bool>,
    pub need_fire_money: Option<usize>,
    pub origin_need_fire_money: Option<usize>,
    pub tsukkomi: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chapter {
    pub add_time: String,
    pub audit_status: i32,
    pub chap_id: i32,
    pub chap_order: i32,
    pub char_count: usize,
    pub expand: Option<ChapterExpand>,
    pub is_rubbish: bool,
    pub is_vip: bool,
    pub novel_id: i32,
    pub ntitle: String,
    pub row_num: usize,
    pub sno: f32,
    pub title: String,
    pub update_time: Option<String>,
    pub volume_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterInfo {
    #[serde(rename = "UpperCamelCase")]
    pub add_time: String,
    pub audit_status: i32,
    pub can_unlock_with_ad: bool,
    pub chap_id: i32,
    pub chap_order: i32,
    pub chapter_origin_fire_money: usize,
    pub char_count: usize,
    pub content: Option<String>,
    pub is_rubbish: bool,
    pub is_vip: bool,
    pub need_fire_money: usize,
    pub novel_id: i32,
    pub ntitle: String,
    pub origin_need_fire_money: usize,
    pub row_num: usize,
    pub sno: f32,
    pub title: String,
    pub update_time: Option<String>,
    pub volume_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeInfoV1 {
    pub chapter_list: Vec<ChapterInfo>,
    pub sno: f32,
    pub title: String,
    pub volume_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeInfoV2 {
    pub sno: f32,
    pub title: String,
    pub volume_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalogue {
    pub last_update_time: String,
    pub novel_id: i32,
    pub volume_list: Vec<VolumeInfoV1>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NovelRef {
    pub allow_down: bool,
    pub author_id: i32,
    pub author_name: String,
    pub bg_banner: String,
    pub category_id: i32,
    pub char_count: usize,
    pub expand: Option<serde_json::Value>,
    pub is_finish: bool,
    pub is_sensitive: bool,
    pub is_sticky: bool,
    pub last_update_time: String,
    pub mark_count: usize,
    pub mark_date_time: String,
    pub novel_cover: String,
    pub novel_id: i32,
    pub novel_name: String,
    pub point: f32,
    pub sign_status: String,
    pub sticky_date_time: Option<String>,
    pub type_id: i32,
    pub view_times: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicRef {
    pub author_id: i32,
    pub bg_banner: String,
    pub comic_cover: String,
    pub comic_id: i32,
    pub comic_name: String,
    pub folder_name: String,
    pub is_finished: bool,
    pub is_sticky: bool,
    pub last_update_time: String,
    pub latest_chapter_title: String,
    pub mark_date_time: String,
    pub point: f32,
    pub sign_status: String,
    pub sticky_date_time: Option<String>,
    pub type_id: i32,
    pub view_times: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumRef {
    pub album_id: i32,
    pub author_id: i32,
    pub cover_big: String,
    pub cover_medium: String,
    pub cover_small: String,
    pub is_sticky: bool,
    pub last_update_time: String,
    pub latest_chapter_id: i32,
    pub name: String,
    pub novel_id: i32,
    pub sticky_date_time: Option<String>,
    pub visit_times: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PocketExpand {
    pub albums: Option<Vec<AlbumRef>>,
    pub comics: Option<Vec<ComicRef>>,
    pub novels: Option<Vec<NovelRef>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pocket {
    pub account_id: i32,
    pub can_modify: bool,
    pub create_time: String,
    pub expand: Option<PocketExpand>,
    pub is_full: bool,
    pub name: String,
    pub pocket_id: i32,
    pub type_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckInInfo {
    continue_num: i32,
    day: i32,
    month: i32,
    year: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NovelRecord {
    pub add_time: String,
    pub allow_down: bool,
    pub author_id: i32,
    pub author_name: String,
    pub bg_banner: String,
    pub category_id: i32,
    pub char_count: usize,
    pub is_finish: bool,
    pub is_sensitive: bool,
    pub last_update_time: String,
    pub mark_count: usize,
    pub novel_cover: String,
    pub novel_id: i32,
    pub novel_name: String,
    pub point: f32,
    pub sign_status: String,
    pub type_id: i32,
    pub view_times: usize,
    pub weight: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub albums: Vec<AlbumRef>,
    pub comics: Vec<ComicRef>,
    pub novels: Vec<NovelRecord>,
}
