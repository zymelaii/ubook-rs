use chrono::NaiveDateTime;

use super::*;

impl From<types::UserPrivate> for UserInfo {
    fn from(value: types::UserPrivate) -> Self {
        Self {
            user_id: value.accountId,
            nickname: value.nickName,
            avatar: value.avatar,
            is_author: value.isAuthor,
            private_info: Some(UserInfoPrivate {
                internal_id: value.userName,
                phone_number: value.phoneNum,
                email: value.email,
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

impl From<types::User> for UserInfo {
    fn from(value: types::User) -> Self {
        if let Some(expand) = value.expand {
            Self {
                user_id: value.accountId,
                nickname: value.nickName,
                intro: expand.introduction.unwrap_or_default(),
                avatar: expand.avatar.unwrap_or_default(),
                total_follows: expand.followNum.unwrap_or_default(),
                total_fans: expand.fansNum.unwrap_or_default(),
                private_info: Some(UserInfoPrivate {
                    internal_id: value.userName,
                    ..Default::default()
                }),
                ..Default::default()
            }
        } else {
            Self {
                user_id: value.accountId,
                nickname: value.nickName,
                private_info: Some(UserInfoPrivate {
                    internal_id: value.userName,
                    ..Default::default()
                }),
                ..Default::default()
            }
        }
    }
}

impl From<types::Novel> for NovelInfo {
    fn from(value: types::Novel) -> Self {
        fn to_timestamp(s: &str) -> crate::Result<Timestamp> {
            Ok(NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")?
                .timestamp_millis()
                .into())
        }

        let mut novel_info = Self {
            novel_id: value.novelId.unwrap(),
            author_id: value.authorId,
            name: value.novelName,
            author: value.authorName,
            total_chars: value.charCount,
            is_finished: value.isFinish,
            creation_date: to_timestamp(&value.addTime).unwrap_or_default(),
            last_update_date: to_timestamp(&value.lastUpdateTime).unwrap_or_default(),
            cover: value.novelCover,
            banner: Some(value.bgBanner),
            sign_info: SignInfo {
                is_signed: value.signStatus != "普通",
                ..Default::default()
            },
            browse_info: BrowseInfo {
                total_views: value.viewTimes,
                ..Default::default()
            },
            ..Default::default()
        };

        if let Some(expand) = value.expand {
            novel_info.intro = expand.intro.unwrap_or_default();
            novel_info.r#type = expand.typeName.unwrap_or_default();

            let sign_level = expand.signLevel.unwrap_or_default();
            novel_info.sign_info.level = match sign_level.as_str() {
                "normal" => 1,
                "vipB" => 2,
                "vipA" => 3,
                "vipS" => 4,
                _ => 0,
            };
            novel_info.sign_info.extra = sign_level;

            novel_info.browse_info.total_likes = expand.fav.unwrap_or_default();
            novel_info.browse_info.total_votes = expand.fav.unwrap_or_default();
        }

        novel_info
    }
}

impl From<types::VolumeInfoV2> for VolumeInfo {
    fn from(value: types::VolumeInfoV2) -> Self {
        Self {
            volume_id: value.volumeId,
            title: value.title,
            order: value.sno as usize,
            ..Default::default()
        }
    }
}

impl From<types::Chapter> for ChapterInfo {
    fn from(value: types::Chapter) -> Self {
        fn to_timestamp(s: &str) -> crate::Result<Timestamp> {
            Ok(NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")?
                .timestamp_millis()
                .into())
        }

        let pub_date = to_timestamp(&value.addTime).unwrap_or_default();

        Self {
            novel_id: value.novelId,
            volume_id: value.volumeId,
            chapter_id: value.chapId,
            title: value.title,
            order: value.chapOrder.try_into().unwrap(),
            total_words: value.charCount,
            pub_date,
            rev_date: value
                .updateTime
                .and_then(|time| to_timestamp(&time).ok())
                .unwrap_or(pub_date),
            price_info: value.expand.as_ref().map(|e| PriceInfo {
                is_free: e.needFireMoney.map(|e| e == 0).unwrap_or(false),
                original_price: e.originNeedFireMoney.unwrap_or_default() as f32,
                sale_price: e.needFireMoney.map(|price| price as f32),
            }),
            content: value.expand.and_then(|e| e.content),
        }
    }
}
