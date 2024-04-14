use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub longitude: Option<String>,
    #[serde(default)]
    pub latitude: Option<String>,
    #[serde(default)]
    pub postcode: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(rename = "type")]
    pub location_type: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}