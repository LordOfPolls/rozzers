use crate::models::forces::{Force};
use crate::models::officer::Officer;
use crate::BASE_URL;
use crate::errors::Error;
use crate::requests::base_request;

pub async fn list_all_forces() -> Result<Vec<Force>, Error> {
    let url = format!("{}/forces", BASE_URL);

    let response = base_request(&url, None).await?;

    if response.status().is_success() {
        let forces: Vec<Force> = response.json().await?;
        return Ok(forces);
    }

    Err(Error::APIError("No forces found".to_string()))
}

pub async fn get_specific_force(force_id: &str) -> Result<Force, Error> {
    let url = format!("{}/forces/{}", BASE_URL, force_id);

    let response = base_request(&url, None).await?;

    if response.status().is_success() {
        let force: Force = response.json().await?;
        return Ok(force);
    }

    Err(Error::APIError("Force not found".to_string()))
}

pub async fn get_force_senior_officers(force_id: &str) -> Result<Vec<Officer>, Error> {
    let url = format!("{}/forces/{}/people", BASE_URL, force_id);

    let response = base_request(&url, None).await?;

    if response.status().is_success() {
        let officers: Vec<Officer> = response.json().await?;
        return Ok(officers);
    }

    Err(Error::APIError("No officers found".to_string()))
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test(tokio::test)]
    async fn test_list_all_forces() {
        let forces = list_all_forces().await;

        assert!(forces.is_ok());

        assert!(forces.unwrap().len() > 0);
    }

    #[test(tokio::test)]
    async fn test_get_specific_force() {
        let force = get_specific_force("avon-and-somerset").await;

        assert!(force.is_ok());

        assert_eq!(force.unwrap().id, "avon-and-somerset");
    }

    #[test(tokio::test)]
    async fn test_get_force_senior_officers() {
        let officers = get_force_senior_officers("leicestershire").await;

        assert!(officers.is_ok());

        assert!(officers.unwrap().len() > 0);
    }
}