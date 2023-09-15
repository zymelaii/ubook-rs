use super::*;

/// 小说章节信息
#[derive(Debug, Default)]
pub struct ChapterInfo {
    /// 小说 ID
    pub novel_id: Id,
    /// 分卷 ID
    pub volume_id: Id,
    /// 章节 ID
    pub chapter_id: Id,
    /// 章节标题
    pub title: String,
    /// 章节序号
    pub order: usize,
    /// 总字数
    pub total_words: usize,
    /// 发布时间
    pub pub_date: Timestamp,
    /// 最后修订时间
    pub rev_date: Timestamp,
    /// 章节内容
    pub content: Option<String>,
    /// 章节价格信息
    pub price_info: Option<PriceInfo>,
}

/// 小说分卷信息
#[derive(Debug, Default)]
pub struct VolumeInfo {
    /// 小说 ID
    pub novel_id: Id,
    /// 分卷 ID
    pub volume_id: Id,
    /// 分卷名称
    pub title: String,
    /// 分卷序号
    pub order: usize,
    /// 章节列表
    pub chapters: Vec<Id>,
}

/// 小说信息
#[derive(Debug, Default)]
pub struct NovelInfo {
    /// 小说 ID
    pub novel_id: Id,
    /// 作者 ID
    pub author_id: Id,
    /// 作品名
    pub name: String,
    /// 作者笔名
    pub author: String,
    /// 简介
    pub intro: String,
    /// 小说题材
    pub r#type: String,
    /// 总字数
    pub total_chars: usize,
    /// 是否完结
    pub is_finished: bool,
    /// 创建时间
    pub creation_date: Timestamp,
    /// 最后一次更新的时间
    pub last_update_date: Timestamp,
    /// 封面 URL
    pub cover: Url,
    /// 背景横幅 URL
    pub banner: Option<Url>,
    /// 分卷列表
    pub volumes: Vec<Id>,
    /// 签约信息
    pub sign_info: SignInfo,
    /// 浏览信息
    pub browse_info: BrowseInfo,
}
