use phf::{phf_map, Map};

pub const APIPREFIX: &'static str = "https://api.sfacg.com";

pub const AUTH: &'static str = "Basic YW5kcm9pZHVzZXI6MWEjJDUxLXl0Njk7KkFjdkBxeHE=";

pub const APPKEYS: Map<&'static str, &'static str> = phf_map! {
    "4.8.42(android;25)" => "FMLxgOdsfxmN!Dt4",
};

pub const FULLEXPAND: Map<&'static str, &'static str> = phf_map! {
    "novels" => "chapterCount,bigBgBanner,bigNovelCover,typeName,intro,fav,ticket,pointCount,tags,sysTags,signlevel,discount,discountExpireDate,totalNeedFireMoney,rankinglist,originTotalNeedFireMoney,firstchapter,latestchapter,latestcommentdate,essaytag,auditCover,preOrderInfo,customTag,topic,unauditedCustomtag,homeFlag,isbranch",
    "novels/dirs" => "originNeedFireMoney",
    "user" => "introduction,bigAvatar,avatar,backgroundPic,fansNum,followNum,followyou,youfollow,verifyType,verifyInfo,avatarFrame,youblock,widgets,growup",
    "user/private" => "changeNickNameInfo,hasActiveUnlockChapWithAd,hasOrderChapWithFireMoney,hasOrderedVipChaps,hasPaidFirstTime,hasUnlockChapWithAd,isRealNameAuth,redpacketCode,usedRedpacketCode,useWelfaresys,vipInfo,welfareCoin,welfareMoney",
    "user/pockets" => "comics,novels,albums",
};
