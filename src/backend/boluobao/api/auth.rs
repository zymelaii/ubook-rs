use super::*;
use crate::RequestBuilder;

use async_trait::async_trait;
use reqwest::{header::*, StatusCode};
use serde_json::json;

#[async_trait]
impl crate::api::AuthAPI for BoluobaoHost {
    async fn authenticate(&mut self, account: &str, password: &str) -> crate::Result<String> {
        let secrets = json!({
            "username": account,
            "password": password,
        });

        let resp = self
            .as_guest()
            .api_post("/sessions")
            .body(secrets.to_string())
            .send()
            .await?;

        let auth_cookies = [".SFCommunity", "session_APP"];
        let cookies = resp
            .cookies()
            .filter(|cookie| auth_cookies.contains(&cookie.name()))
            .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
            .collect::<Vec<String>>()
            .join("; ");

        process_response::<()>(resp.status(), resp.text().await?.as_str())?;

        Ok(cookies)
    }

    async fn query_auth_status(&mut self, user_id: Id) -> crate::Result<()> {
        if let Some(host) = self.as_auth(user_id) {
            let resp = host.api_get("/user").send().await?;
            if resp.status() == StatusCode::OK {
                Ok(())
            } else {
                anyhow::bail!("expired session")
            }
        } else {
            anyhow::bail!("unauthorized user")
        }
    }

    async fn login(&mut self, account: &str, password: &str) -> crate::Result<Id> {
        let cookie = self.authenticate(account, password).await?;
        let resp = self
            .as_guest()
            .api_get("/user")
            .header(COOKIE, &cookie)
            .send()
            .await?;

        let data: types::UserPrivate =
            process_response(resp.status(), resp.text().await?.as_str())?
                .expect("missing expected field");
        let user_id = UserInfo::from(data).user_id;

        let mut session = SessionInfo::default();
        cookie.split("; ").for_each(|item| {
            let (key, value) = item.split_once('=').expect("invalid set-cookie header");
            match key {
                ".SFCommunity" => session.token = value.to_string(),
                "session_APP" => session.sid = value.to_string(),
                _ => (),
            }
        });
        self.update_auth(user_id, session);

        Ok(user_id)
    }

    async fn logout(&mut self, user_id: Id) -> crate::Result<()> {
        self.remove_auth(user_id);
        Ok(())
    }
}
