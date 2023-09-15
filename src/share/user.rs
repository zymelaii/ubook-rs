use super::*;

/// 用户私人信息
#[derive(Debug, Default)]
pub struct UserInfoPrivate {
    /// 内部用户标识符
    pub internal_id: String,
    /// 账户
    pub account: String,
    /// 密码
    pub password: String,
    /// 绑定的手机号码
    pub phone_number: String,
    /// 绑定的邮箱
    pub email: String,
}

/// 用户信息
#[derive(Debug, Default)]
pub struct UserInfo {
    /// 用户 ID
    pub user_id: Id,
    /// 用户昵称
    pub nickname: String,
    /// 个人简介
    pub intro: String,
    /// 头像 URL
    pub avatar: Url,
    /// 总点赞量
    pub total_likes: usize,
    /// 总关注数
    pub total_follows: usize,
    /// 总粉丝数
    pub total_fans: usize,
    /// 是否为作者
    pub is_author: bool,
    /// 关联作者 ID
    pub author_id: Option<Id>,
    /// 私人信息
    pub private_info: Option<UserInfoPrivate>,
}
