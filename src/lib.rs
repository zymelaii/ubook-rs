pub use anyhow::Result;

pub mod api;
pub mod backend;
pub mod cli;
pub mod share;
pub mod storage;

mod host;
pub use host::BackendManager;

mod request;
pub use request::RequestBuilder;

pub trait BackendCore:
    RequestBuilder + api::UserAPI + api::NovelAPI + api::AuthorAPI + api::SearchAPI
{
    fn backend_id(&self) -> &'static str;
}

pub trait Backend: BackendCore + api::AuthAPI {}

#[derive(Default)]
pub struct UbookContext {
    pub backend_manager: BackendManager,
}

impl UbookContext {
    pub fn new() -> Self {
        Self::default()
    }
}
