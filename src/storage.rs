use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BackendStatus {
    pub id: String,
    pub active: bool,
    pub enabled: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContextStorage {
    pub backends: Vec<BackendStatus>,
}
