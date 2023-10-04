use super::*;

/// 作品类型
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum WorkType {
    /// 未知类型
    Unknown,
    /// 小说
    Novel,
    /// 漫画
    Comic,
    /// 听书
    Audiobook,
    /// 短故事
    ShortStory,
}

impl Default for WorkType {
    fn default() -> Self {
        Self::Unknown
    }
}

/// 作品引用
#[derive(Debug)]
pub struct WorkRef {
    /// 作品类型
    pub r#type: WorkType,
    /// 作品 ID
    pub work_id: Id,
}

/// 作品检索记录
#[derive(Debug, Default)]
pub struct WorkSearchResult {
    /// 作品类型
    pub r#type: WorkType,
    /// 作品 ID
    pub work_id: Id,
    /// 作者 ID
    pub author_id: Id,
    /// 作品名称
    pub work_name: String,
    /// 作者名称
    pub author_name: String,
    /// 作品封面
    pub cover: Url,
    /// 作品简介
    pub intro: String,
    /// 作品标签
    pub tags: Vec<String>,
    /// 作品热度
    pub popularity: usize,
}

/// 价格信息
#[derive(Debug, Default)]
pub struct PriceInfo {
    /// 是否免费
    pub is_free: bool,
    /// 原价
    pub original_price: f32,
    /// 现价
    pub sale_price: Option<f32>,
}

/// 作品浏览信息
#[derive(Debug, Default)]
pub struct BrowseInfo {
    /// 总浏览量
    pub total_views: usize,
    /// 总点赞数
    pub total_likes: usize,
    /// 总收藏数
    pub total_favs: usize,
    /// 总打赏量
    pub total_rewards: usize,
    /// 总投票数
    pub total_votes: usize,
}

/// 签约信息
#[derive(Debug, Default)]
pub struct SignInfo {
    /// 是否签约
    pub is_signed: bool,
    /// 签约等级
    pub level: usize,
    /// 附加信息
    pub extra: String,
}
