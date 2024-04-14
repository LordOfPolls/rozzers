use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::utils::deserialize_date;
#[derive(Serialize, Deserialize, Debug)]
pub struct Availability {
    // todo: this is in ISO_8601 format, needs to be a date time
    /// Year and month of all available street level data
    #[serde(deserialize_with = "deserialize_date")]
    pub date: NaiveDate,
    /// List of force IDs for forces that have provided stop and search data for this month
    #[serde(default)]
    pub forces: Vec<String>,
}