use chrono::NaiveDateTime;

use super::*;

impl From<types::UserPrivate> for UserInfo {
    fn from(value: types::UserPrivate) -> Self {
        Self {
            user_id: value.account_id,
            nickname: value.nick_name,
            avatar: value.avatar,
            is_author: value.is_author,
            private_info: Some(UserInfoPrivate {
                internal_id: value.user_name,
                phone_number: value.phone_num,
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
                user_id: value.account_id,
                nickname: value.nick_name,
                intro: expand.introduction.unwrap_or_default(),
                avatar: expand.avatar.unwrap_or_default(),
                total_follows: expand.follow_num.unwrap_or_default(),
                total_fans: expand.fans_num.unwrap_or_default(),
                private_info: Some(UserInfoPrivate {
                    internal_id: value.user_name,
                    ..Default::default()
                }),
                ..Default::default()
            }
        } else {
            Self {
                user_id: value.account_id,
                nickname: value.nick_name,
                private_info: Some(UserInfoPrivate {
                    internal_id: value.user_name,
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
            novel_id: value.novel_id.unwrap(),
            author_id: value.author_id,
            name: value.novel_name,
            author: value.author_name,
            total_chars: value.char_count,
            is_finished: value.is_finish,
            creation_date: to_timestamp(&value.add_time).unwrap_or_default(),
            last_update_date: to_timestamp(&value.last_update_time).unwrap_or_default(),
            cover: value.novel_cover,
            banner: Some(value.bg_banner),
            sign_info: SignInfo {
                is_signed: value.sign_status != "普通",
                ..Default::default()
            },
            browse_info: BrowseInfo {
                total_views: value.view_times,
                ..Default::default()
            },
            ..Default::default()
        };

        if let Some(expand) = value.expand {
            novel_info.intro = expand.intro.unwrap_or_default();
            novel_info.r#type = expand.type_name.unwrap_or_default();

            let sign_level = expand.sign_level.unwrap_or_default();
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
            volume_id: value.volume_id,
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

        let pub_date = to_timestamp(&value.add_time).unwrap_or_default();

        Self {
            novel_id: value.novel_id,
            volume_id: value.volume_id,
            chapter_id: value.chap_id,
            title: value.title,
            order: value.chap_order.try_into().unwrap(),
            total_words: value.char_count,
            pub_date,
            rev_date: value
                .update_time
                .and_then(|time| to_timestamp(&time).ok())
                .unwrap_or(pub_date),
            price_info: value.expand.as_ref().map(|e| PriceInfo {
                is_free: e.need_fire_money.map(|e| e == 0).unwrap_or(false),
                original_price: e.origin_need_fire_money.unwrap_or_default() as f32,
                sale_price: e.need_fire_money.map(|price| price as f32),
            }),
            content: value.expand.and_then(|e| e.content),
        }
    }
}
