use super::types;
use crate::share::*;

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
        match value.expand {
            Some(expand) => Self {
                user_id: value.accountId,
                nickname: value.nickName,
                intro: expand.introduction.unwrap_or(String::default()),
                avatar: expand.avatar.unwrap_or(Default::default()),
                total_follows: expand.followNum.unwrap_or(Default::default()),
                total_fans: expand.fansNum.unwrap_or(Default::default()),
                private_info: Some(UserInfoPrivate {
                    internal_id: value.userName,
                    ..Default::default()
                }),
                ..Default::default()
            },
            None => Self {
                user_id: value.accountId,
                nickname: value.nickName,
                private_info: Some(UserInfoPrivate {
                    internal_id: value.userName,
                    ..Default::default()
                }),
                ..Default::default()
            },
        }
    }
}
