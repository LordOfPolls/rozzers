use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    /// Unique identifier for the category
    #[serde(rename = "url")]
    pub id: String,
    /// Name of the category
    pub name: String,
}