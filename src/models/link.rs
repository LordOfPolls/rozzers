use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    /// URL
    pub url: String,
    /// Description
    pub description: Option<String>,
    /// Title
    pub title: String,
}