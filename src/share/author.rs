use super::*;

/// 作者信息
#[derive(Debug, Default)]
pub struct AuthorInfo {
    /// 作者 ID
    pub author_id: Id,
    /// 关联用户 ID
    pub user_id: Id,
    /// 作者笔名
    pub name: String,
    /// 作者介绍
    pub intro: String,
    /// 总粉丝数
    pub total_fans: usize,
    /// 累计创作天数
    pub total_worked_days: usize,
    /// 作品列表
    pub work_list: Vec<WorkRef>,
}
