use super::*;

/// 作品类型
#[derive(Debug)]
pub enum WorkType {
    /// 小说
    Novel,
    /// 漫画
    Comic,
    /// 听书
    Audiobook,
    /// 短故事
    ShortStory,
}

/// 作品引用
#[derive(Debug)]
pub struct WorkRef {
    /// 作品类型
    pub r#type: WorkType,
    /// 作品 ID
    pub work_id: Id,
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
    pub is_signed: String,
    /// 签约等级
    pub level: usize,
    /// 附加信息
    pub extra: String,
}
