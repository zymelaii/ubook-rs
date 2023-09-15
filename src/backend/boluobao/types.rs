use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ChangeNickNameInfo {
    pub canChange: bool,
    pub nextChangeNeedDays: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct VipInfo {
    pub discount: i32,
    pub nextDiscount: i32,
    pub level: usize,
    pub nextLevel: usize,
    pub nextDiscountLevel: usize,
    pub nextLevelPoint: usize,
    pub nextDiscountLevelPoint: usize,
    pub point: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct UserPrivateExpand {
    pub changeNickNameInfo: Option<ChangeNickNameInfo>,
    pub hasActiveUnlockChapWithAd: Option<bool>,
    pub hasOrderChapWithFireMoney: Option<bool>,
    pub hasOrderedVipChaps: Option<bool>,
    pub hasPaidFirstTime: Option<bool>,
    pub hasUnlockChapWithAd: Option<bool>,
    pub isRealNameAuth: Option<bool>,
    pub redpacketCode: Option<String>,
    pub useWelfaresys: Option<bool>,
    pub usedRedpacketCode: Option<String>,
    pub vipInfo: Option<VipInfo>,
    pub welfareCoin: Option<i32>,
    pub welfareMoney: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct UserPrivate {
    pub accountId: i32,
    pub nickName: String,
    pub userName: String,
    pub countryCode: u32,
    pub avatar: String,
    pub email: String,
    pub fireCoin: i32,
    pub isAuthor: bool,
    pub phoneNum: String,
    pub registerDate: String,
    pub roleName: String,
    pub expand: Option<UserPrivateExpand>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct UserExpand {
    pub avatar: Option<String>,
    pub introduction: Option<String>,
    pub followNum: Option<usize>,
    pub youblock: Option<bool>,
    pub youfollow: Option<bool>,
    pub followyou: Option<bool>,
    pub verifyInfo: Option<String>,
    pub verifyType: Option<i32>,
    pub avatarFrame: Option<String>,
    pub backgroundPic: Option<String>,
    pub bigAvatar: Option<String>,
    pub fansNum: Option<usize>,
    pub widgets: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct User {
    pub accountId: i32,
    pub nickName: String,
    pub userName: String,
    pub expand: Option<UserExpand>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SystemTag {
    sysTagId: i32,
    tagName: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RankingList {
    dateRange: i32,
    desc: String,
    r#type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ChapterRef {
    addTime: String,
    chapId: i32,
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct NovelExpand {
    pub intro: Option<String>,
    pub signLevel: Option<String>,
    pub chapterCount: Option<usize>,
    pub auditCover: Option<String>,
    pub bigBgBanner: Option<String>,
    pub bigNovelCover: Option<String>,
    pub customTag: Option<Vec<String>>,
    pub firstChapter: Option<ChapterRef>,
    pub lastChapter: Option<ChapterRef>,
    pub discount: Option<f32>,
    pub discountExpireDate: Option<String>,
    pub fav: Option<usize>,
    pub essayTag: Option<String>,
    pub unauditedCustomtag: Option<Vec<String>>,
    pub homeFlag: Option<Vec<String>>,
    pub isBanch: Option<bool>,
    pub latestCommentDate: Option<String>,
    pub pointCount: Option<usize>,
    pub preOrderInfo: Option<String>,
    pub rankinglist: Option<RankingList>,
    pub sysTags: Option<Vec<SystemTag>>,
    pub tags: Option<Vec<String>>,
    pub topic: Option<String>,
    pub ticket: Option<usize>,
    pub typeName: Option<String>,
    pub originTotalNeedFireMoney: Option<usize>,
    pub totalNeedFireMoney: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Novel {
    pub addTime: String,
    pub allowDown: bool,
    pub novelId: Option<i32>,
    pub authorId: i32,
    pub authorName: String,
    pub bgBanner: String,
    pub categoryId: i32,
    pub charCount: usize,
    pub isFinish: bool,
    pub isSensitive: bool,
    pub lastUpdateTime: String,
    pub markCount: usize,
    pub novelName: String,
    pub point: f32,
    pub signStatus: String,
    pub typeId: usize,
    pub viewTimes: usize,
    pub expand: Option<NovelExpand>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Chapter {
    pub novelId: i32,
    pub volumeId: i32,
    pub chapId: i32,

    pub title: String,
    pub ntitle: String,

    pub sno: f32,
    pub chapOrder: i32,

    pub rowNum: usize,
    pub charCount: usize,

    pub isVip: bool,
    pub isRubbish: bool,
    pub auditStatus: i32,

    pub AddTime: String,
    pub updateTime: Option<String>,

    pub needFireMoney: usize,
    pub originNeedFireMoney: usize,
    pub chapterOriginFireMoney: usize,
    pub canUnlockWithAd: bool,

    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Volume {
    pub volumeId: i32,
    pub sno: f32,
    pub title: String,
    pub chapterList: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Catalogue {
    pub novelId: i32,
    pub lastUpdateTime: String,
    pub volumeList: Vec<Volume>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct NovelRef {
    pub allowDown: bool,
    pub authorId: i32,
    pub authorName: String,
    pub bgBanner: String,
    pub categoryId: i32,
    pub charCount: usize,
    pub isFinish: bool,
    pub isSensitive: bool,
    pub isSticky: bool,
    pub lastUpdateTime: String,
    pub markCount: usize,
    pub markDateTime: String,
    pub novelCover: String,
    pub novelId: i32,
    pub novelName: String,
    pub point: f32,
    pub signStatus: String,
    pub stickyDateTime: Option<String>,
    pub typeId: i32,
    pub viewTimes: usize,
    pub expand: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ComicRef {
    pub authorId: i32,
    pub bgBanner: String,
    pub comicCover: String,
    pub comicId: i32,
    pub comicName: String,
    pub folderName: String,
    pub isFinished: bool,
    pub isSticky: bool,
    pub lastUpdateTime: String,
    pub latestChapterTitle: String,
    pub point: f32,
    pub markDateTime: String,
    pub signStatus: String,
    pub stickyDateTime: Option<String>,
    pub typeId: i32,
    pub viewTimes: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AlbumRef {
    pub authorId: i32,
    pub novelId: i32,
    pub albumId: i32,
    pub name: String,
    pub latestChapterId: i32,
    pub lastUpdateTime: String,
    pub coverSmall: String,
    pub coverMedium: String,
    pub coverBig: String,
    pub isSticky: bool,
    pub stickyDateTime: Option<String>,
    pub visitTimes: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PocketExpand {
    pub comics: Option<Vec<ComicRef>>,
    pub novels: Option<Vec<NovelRef>>,
    pub albums: Option<Vec<AlbumRef>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Pocket {
    pub accountId: i32,
    pub canModify: bool,
    pub createTime: String,
    pub isFull: bool,
    pub name: String,
    pub pocketId: i32,
    pub typeId: i32,
    pub expand: Option<PocketExpand>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct CheckInInfo {
    year: i32,
    month: i32,
    day: i32,
    continueNum: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct NovelRecord {
    pub allowDown: bool,
    pub authorId: i32,
    pub authorName: String,
    pub addTime: String,
    pub bgBanner: String,
    pub categoryId: i32,
    pub charCount: usize,
    pub isFinish: bool,
    pub isSensitive: bool,
    pub lastUpdateTime: String,
    pub markCount: usize,
    pub novelCover: String,
    pub novelId: i32,
    pub novelName: String,
    pub point: f32,
    pub signStatus: String,
    pub typeId: i32,
    pub viewTimes: usize,
    pub weight: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SearchResult {
    pub albums: Vec<AlbumRef>,
    pub comics: Vec<ComicRef>,
    pub novels: Vec<NovelRecord>,
}
