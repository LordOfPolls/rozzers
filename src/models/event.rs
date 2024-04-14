use chrono::{NaiveDate, NaiveDateTime};
use crate::models::contact_details::ContactDetails;
use serde::{Deserialize, Serialize};
use crate::utils::option_deserialize_datetime;

#[derive(Serialize, Deserialize)]
pub struct Event {
    /// Contact details for the event
    #[serde(default)]
    pub contact_details: ContactDetails,

    /// Description of the event
    #[serde(default)]
    pub description: Option<String>,

    /// Title of the event
    #[serde(default)]
    pub title: Option<String>,

    /// Address of the event
    #[serde(default)]
    pub address: Option<String>,

    /// type of event
    #[serde(rename = "type")]
    #[serde(default)]
    pub event_type: Option<String>,

    // todo: these are in ISO_8601, need to be converted to DateTime
    /// Start date of the event
    #[serde(default)]
    #[serde(deserialize_with = "option_deserialize_datetime")]
    pub start_date: Option<NaiveDateTime>,
    /// End date of the event
    #[serde(default)]
    #[serde(deserialize_with = "option_deserialize_datetime")]
    pub end_date: Option<NaiveDateTime>,
}