use super::share::*;

pub trait AuthAPI {
    /// 验证账户身份
    fn try_auth(&mut self, account: &str, password: &str) -> crate::Result<String>;

    /// 获取当前身份验证状态
    fn query_auth_status(&mut self, user_id: Id) -> crate::Result<()>;

    /// 登录账户
    fn try_login(&mut self, account: &str, password: &str) -> crate::Result<Id>;

    /// `需要登陆` 注销活动账户
    ///
    /// 仅退出当前账户并结束会话，注销之后必须重新登陆账户
    fn try_logout(&mut self, user_id: Id) -> crate::Result<()>;
}

pub trait UserAPI {
    /// 查询用户信息（不含隐私信息）
    fn query_user_info(&mut self, user_id: Id) -> crate::Result<UserInfo>;

    /// `需要登陆` 获取个人资料
    fn try_get_profile(&mut self, user_id: Id, with_private: bool) -> crate::Result<UserInfo>;
}

pub trait AuthorAPI {
    /// 查询作者信息
    fn query_author_info(&mut self, author_id: Id) -> crate::Result<AuthorInfo>;

    /// 查询作者的作品列表
    fn query_work_list(&mut self, author_id: Id) -> crate::Result<Vec<WorkRef>>;
}

pub trait NovelAPI {
    /// 查询小说信息
    fn query_novel_info(&mut self, novel_id: Id) -> crate::Result<NovelInfo>;

    /// 查询分卷信息
    fn query_volume_info(&mut self, volume_id: Id) -> crate::Result<VolumeInfo>;

    /// 查询章节信息（不含章节正文）
    fn query_chapter_info(&mut self, chapter_id: Id) -> crate::Result<ChapterInfo>;

    /// `可选登录` 获取章节正文
    ///
    /// 当目标章节不存在或章节未订阅时获取失败
    fn try_get_chapter_content(&mut self, chapter_id: Id) -> crate::Result<String>;
}
