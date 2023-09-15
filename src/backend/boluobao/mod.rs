mod adapter;
mod api;
mod internal;
pub mod types;

pub use adapter::*;
pub use api::*;
pub use internal::*;

use crate::share::*;
use reqwest::{
    blocking::{Client, RequestBuilder},
    header::{HeaderMap, *},
    IntoUrl, Method,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SessionInfo {
    pub token: String,
    pub sid: String,
}

#[derive(Debug)]
pub struct BoluobaoHost {
    client: Client,
    device_token: String,
    active_user: Option<Id>,
    sessions: HashMap<Id, SessionInfo>,
}

impl BoluobaoHost {
    pub fn new() -> Self {
        Self {
            client: Default::default(),
            device_token: Uuid::new_v4().to_string().to_lowercase(),
            active_user: None,
            sessions: Default::default(),
        }
    }

    pub fn update_auth(&mut self, user_id: Id, session: SessionInfo) {
        self.sessions.insert(user_id, session);
    }

    pub fn remove_auth(&mut self, user_id: Id) {
        self.sessions.remove(&user_id);
        if let Some(uid) = self.active_user {
            if uid == user_id {
                self.active_user = None;
            }
        }
    }
}

impl BoluobaoHost {
    fn as_guest(&mut self) -> &Self {
        self.active_user = None;
        self
    }

    fn as_auth(&mut self, user_id: Id) -> Option<&Self> {
        if self.sessions.contains_key(&user_id) {
            self.active_user = Some(user_id);
            Some(self)
        } else {
            None
        }
    }

    fn default_header(&self) -> crate::Result<HeaderMap> {
        let app_version = "4.8.42(android;25)";
        let user_agent = format!(
            "boluobao/{}/{}/{}",
            app_version, "HomePage", self.device_token
        );
        let security = get_sfsecurity(app_version, self.device_token.as_str());

        let mut headers = HeaderMap::new();
        headers.append(ACCEPT, "application/vnd.sfacg.api+json;version=1".parse()?);
        headers.append(ACCEPT_CHARSET, "UTF-8".parse()?);
        headers.append(AUTHORIZATION, consts::AUTH.parse()?);
        headers.append(CONTENT_TYPE, "application/json".parse()?);
        headers.append(USER_AGENT, user_agent.parse()?);
        headers.append("SFSecurity", security.parse()?);

        Ok(headers)
    }

    fn request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        let request = self
            .client
            .request(method, url)
            .headers(self.default_header().unwrap_or(HeaderMap::default()));
        if let Some(user_id) = self.active_user {
            let session = self.sessions.get(&user_id).unwrap();
            let cookie = format!(
                ".SFCommunity={}; session_APP={}",
                session.token, session.sid
            );
            request.header(COOKIE, cookie)
        } else {
            request
        }
    }

    pub fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::GET, url)
    }

    pub fn post<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::POST, url)
    }

    pub fn api_get(&self, url: &str) -> RequestBuilder {
        self.request(Method::GET, format!("{}{}", consts::APIPREFIX, url))
    }

    pub fn api_post(&self, url: &str) -> RequestBuilder {
        self.request(Method::POST, format!("{}{}", consts::APIPREFIX, url))
    }
}
