use super::*;
use crate::RequestBuilder;

use async_trait::async_trait;

#[async_trait]
impl crate::api::UserAPI for BoluobaoHost {
    async fn query_user_info(&mut self, user_id: Id) -> crate::Result<UserInfo> {
        let resp = self
            .as_guest()
            .api_get(&format!("/users/{user_id}"))
            .query(&[("expand", "introduction,avatar,followNum,fansNum")])
            .send()
            .await?;

        let data: types::User =
            process_response(resp.status(), &resp.text().await?)?.expect("missing expected field");
        let user_info = UserInfo::from(data);

        Ok(UserInfo {
            private_info: None,
            ..user_info
        })
    }

    async fn get_profile(&mut self, user_id: Id, with_private: bool) -> crate::Result<UserInfo> {
        let basic_user_info = self.query_user_info(user_id).await?;

        if let Some(host) = self.as_auth(user_id) {
            let resp = host.api_get("/user").send().await?;

            let data: types::User = process_response(resp.status(), &resp.text().await?)?
                .expect("missing expected field");
            let user_info = UserInfo::from(data);

            Ok(UserInfo {
                is_author: user_info.is_author,
                private_info: if with_private {
                    user_info.private_info
                } else {
                    None
                },
                ..basic_user_info
            })
        } else {
            anyhow::bail!("unauthorized user")
        }
    }
}
