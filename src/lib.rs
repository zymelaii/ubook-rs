pub use anyhow::Result;

pub mod api;
pub mod backend;
pub mod cli;
pub mod share;

pub trait Backend {
    fn backend_id(&self) -> &'static str;
}
