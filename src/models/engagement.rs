use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngagementMethod {
    /// Method Website URL
    pub url: String,
    /// Type of engagement
    #[serde(rename = "type")]
    pub engagement_type: Option<String>,
    /// Description of the engagement method
    pub description: Option<String>,
    /// Title of the engagement method
    pub title: String,
}