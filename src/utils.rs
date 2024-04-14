use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Deserializer};

pub fn naive_date_from_string(s: String) -> NaiveDate {
    if s.len() == 7 {
        NaiveDate::parse_from_str(&format!("{}-01", s), "%Y-%m-%d").unwrap()
    } else {
        NaiveDate::parse_from_str(&s, "%Y-%m-%d").unwrap()
    }
}

pub fn naive_datetime_from_string(s: String) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S").unwrap()
}

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    Ok(naive_date_from_string(s))
}

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    Ok(naive_datetime_from_string(s))
}

pub fn option_deserialize_datetime<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;

    match s {
        Some(s) => {
            Ok(Some(naive_datetime_from_string(s)))
        }
        None => Ok(None),
    }
}

pub fn option_deserialize_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;

    match s {
        Some(s) => {
            Ok(Some(naive_date_from_string(s)))
        }
        None => Ok(None),
    }
}