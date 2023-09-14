use super::{
    internal::{consts, get_sfsecurity},
    BoluobaoHost,
};
use crate::share::*;
use reqwest::{blocking::Client, header::*, StatusCode};
use serde_json::json;

impl crate::api::AuthAPI for BoluobaoHost {
    fn query_auth_status() -> crate::Result<()> {
        todo!()
    }

    fn try_login(&self, account: &str, password: &str) -> crate::Result<crate::share::Id> {
        let secrets = json!({
            "username": account,
            "password": password,
        });

        let device_token = uuid::Uuid::new_v4().to_string().to_lowercase();
        let app_version = "4.8.42(android;25)";
        let user_agent = format!("boluobao/{}/{}/{}", app_version, "HomePage", device_token);
        let security = get_sfsecurity(app_version, device_token.as_str());

        let client = Client::new();
        let resp = client
            .post(format!("{}/sessions", consts::APIPREFIX))
            .header(ACCEPT, "application/vnd.sfacg.api+json;version=1")
            .header(ACCEPT_CHARSET, "UTF-8")
            .header(AUTHORIZATION, consts::AUTH)
            .header(CONTENT_TYPE, "application/json")
            .header(USER_AGENT, &user_agent)
            .header("SFSecurity", &security)
            .body(secrets.to_string())
            .send()?;

        let status_code = resp.status();
        let cookies = resp
            .headers()
            .get_all(SET_COOKIE)
            .iter()
            .map(|e| {
                let raw = e.to_str().unwrap().to_string();
                let (k, v) = raw
                    .split_once("; ")
                    .unwrap_or(("=", ""))
                    .0
                    .split_once("=")
                    .unwrap();
                (k.to_string(), v.to_string())
            })
            .collect::<Vec<(String, String)>>();
        let data = resp.text()?.parse::<serde_json::Value>()?;

        let tmp = serde_json::Value::default();
        let msg = data
            .get("status")
            .unwrap_or(&tmp)
            .get("msg")
            .unwrap_or(&tmp)
            .as_str()
            .unwrap_or("")
            .to_string();

        match status_code {
            StatusCode::OK => (),
            StatusCode::UNAUTHORIZED | StatusCode::BAD_REQUEST => anyhow::bail!(msg),
            _ => unreachable!(),
        };

        let cookie = cookies
            .iter()
            .map(|e| format!("{}={}", e.0, e.1))
            .collect::<Vec<String>>()
            .join("; ");

        let resp = client
            .get(format!("{}/user", consts::APIPREFIX))
            .header(ACCEPT, "application/vnd.sfacg.api+json;version=1")
            .header(ACCEPT_CHARSET, "UTF-8")
            .header(AUTHORIZATION, consts::AUTH)
            .header(CONTENT_TYPE, "application/json")
            .header(USER_AGENT, &user_agent)
            .header("SFSecurity", &security)
            .header(COOKIE, &cookie)
            .send()?;

        let status_code = resp.status();
        let data = resp.text()?.parse::<serde_json::Value>()?;

        let tmp = serde_json::Value::default();
        let msg = data
            .get("status")
            .unwrap_or(&tmp)
            .get("msg")
            .unwrap_or(&tmp)
            .as_str()
            .unwrap_or("")
            .to_string();

        match status_code {
            StatusCode::OK => (),
            StatusCode::UNAUTHORIZED | StatusCode::BAD_REQUEST => anyhow::bail!(msg),
            _ => unreachable!(),
        };

        let data = data.get("data").unwrap();
        Ok(data.get("accountId").unwrap().as_i64().unwrap() as Id)
    }

    fn try_logout(&self, user_id: crate::share::Id) -> crate::Result<()> {
        let _ = user_id;
        todo!()
    }
}
