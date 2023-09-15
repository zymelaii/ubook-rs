use super::{BoluobaoHost, Response, SessionInfo};
use crate::share::*;
use reqwest::{header::*, StatusCode};
use serde_json::json;

impl crate::api::AuthAPI for BoluobaoHost {
    fn try_auth(&mut self, account: &str, password: &str) -> crate::Result<String> {
        let secrets = json!({
            "username": account,
            "password": password,
        });

        let resp = self
            .as_guest()
            .api_post("/sessions")
            .body(secrets.to_string())
            .send()?;

        let status_code = resp.status();
        match status_code {
            StatusCode::OK => (),
            StatusCode::UNAUTHORIZED | StatusCode::BAD_REQUEST => {
                let data = serde_json::from_str::<Response<()>>(resp.text()?.as_str());
                anyhow::bail!(data
                    .unwrap()
                    .status
                    .msg
                    .unwrap_or(status_code.as_str().to_string()))
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                anyhow::bail!(status_code.as_str().to_string())
            }
            _ => unreachable!(),
        };

        let auth_cookies = [".SFCommunity", "session_APP"];
        let cookies = resp
            .cookies()
            .into_iter()
            .filter_map(|cookie| {
                if auth_cookies.contains(&cookie.name()) {
                    Some(format!("{}={}", cookie.name(), cookie.value()))
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join("; ");

        Ok(cookies)
    }

    fn query_auth_status(&mut self, user_id: Id) -> crate::Result<()> {
        if let Some(host) = self.as_auth(user_id) {
            let resp = host.api_get("/user").send()?;
            if resp.status() == StatusCode::OK {
                Ok(())
            } else {
                anyhow::bail!("expired session")
            }
        } else {
            anyhow::bail!("unauthorized user")
        }
    }

    fn try_login(&mut self, account: &str, password: &str) -> crate::Result<Id> {
        let cookie = self.try_auth(account, password)?;
        let resp = self
            .as_guest()
            .api_get("/user")
            .header(COOKIE, &cookie)
            .send()?;

        let status_code = resp.status();
        let data =
            serde_json::from_str::<Response<super::types::UserPrivate>>(resp.text()?.as_str());

        match status_code {
            StatusCode::OK => (),
            StatusCode::UNAUTHORIZED | StatusCode::BAD_REQUEST => {
                anyhow::bail!(data
                    .unwrap()
                    .status
                    .msg
                    .unwrap_or(status_code.as_str().to_string()))
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                anyhow::bail!(status_code.as_str().to_string())
            }
            _ => unreachable!(),
        };

        let user_id = UserInfo::from(data?.data.unwrap()).user_id;
        let mut session = SessionInfo::default();
        cookie.split("; ").for_each(|item| {
            let (key, value) = item.split_once("=").unwrap();
            match key {
                ".SFCommunity" => session.token = value.to_string(),
                "session_APP" => session.sid = value.to_string(),
                _ => (),
            }
        });
        self.update_auth(user_id, session);

        Ok(user_id)
    }

    fn try_logout(&mut self, user_id: Id) -> crate::Result<()> {
        self.remove_auth(user_id);
        Ok(())
    }
}
