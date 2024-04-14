use chrono::NaiveDate;
use crate::BASE_URL;
use crate::requests::base_request;
use crate::models::availability::Availability;
use crate::errors::Error;
use crate::utils::naive_date_from_string;

pub async fn get_detailed_availability() -> Result<Vec<Availability>, Error> {
    let url = format!("{}/crimes-street-dates", BASE_URL);

    let response = base_request(&url, None).await?;

    if response.status().is_success() {
        let availability: Vec<Availability> = response.json().await?;
        return Ok(availability);
    }

    Err(Error::APIError("No availability found".to_string()))
}

pub async fn get_availability() -> Result<NaiveDate, Error> {
    let url = format!("{}/crime-last-updated", BASE_URL);

    let response = base_request(&url, None).await?;

    if response.status().is_success() {
        let json = response.json::<serde_json::Value>().await?;
        let date = json["date"].as_str().unwrap();
        return Ok(naive_date_from_string(date.to_string()));
    }

    Err(Error::APIError("No availability found".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test(tokio::test)]
    async fn test_get_detailed_availability() {
        let availability = get_detailed_availability().await;

        assert!(availability.is_ok());
    }

    #[test(tokio::test)]
    async fn test_get_availability() {
        let availability = get_availability().await;

        assert!(availability.is_ok());
    }
}