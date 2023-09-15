use super::*;
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

        process_response::<()>(resp.status(), resp.text()?.as_str())?;

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

        let data: types::UserPrivate =
            process_response(resp.status(), resp.text()?.as_str())?.unwrap();
        let user_id = UserInfo::from(data).user_id;

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
