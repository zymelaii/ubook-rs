pub use anyhow::Result;

pub mod api;
pub mod backend;
pub mod cli;
pub mod share;

pub trait BackendCore: api::UserAPI + api::NovelAPI + api::AuthorAPI + api::SearchAPI {
    fn backend_id(&self) -> &'static str;
}

pub trait Backend: BackendCore + api::AuthAPI {}
