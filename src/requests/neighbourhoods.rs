use crate::BASE_URL;
use crate::errors::Error;
use crate::requests::base_request;
use crate::models::neighbourhood::{Neighbourhood, Boundary};
use crate::models::officer::Officer;
use crate::models::event::Event;

pub async fn list_force_neighbourhoods(force_id: &str) -> Result<Vec<Neighbourhood>, Error> {
    let url = format!("{}/{}/neighbourhoods", BASE_URL, force_id);

    let response = base_request(&url, None).await?;

    if response.status().is_success() {
        let mut neighbourhoods: Vec<Neighbourhood> = response.json().await?;
        for neighbourhood in neighbourhoods.iter_mut() {
            neighbourhood.force_id = force_id.to_string();
        }
        return Ok(neighbourhoods);
    }

    Err(Error::APIError("No neighbourhoods found".to_string()))
}

pub async fn get_specific_neighbourhood(force_id: &str, neighbourhood_id: &str) -> Result<Neighbourhood, Error> {
    let url = format!("{}/{}/{}", BASE_URL, force_id, neighbourhood_id);

    let response = base_request(&url, None).await?;

    if response.status().is_success() {
        let mut neighbourhood: Neighbourhood = response.json().await?;
        neighbourhood.force_id = force_id.to_string();
        return Ok(neighbourhood);
    }

    Err(Error::APIError("Neighbourhood not found".to_string()))
}

pub async fn get_neighbourhood_boundaries(force_id: &str, neighbourhood_id: &str) -> Result<Vec<Boundary>, Error> {
    let url = format!("{}/{}/{}/boundary", BASE_URL, force_id, neighbourhood_id);

    let response = base_request(&url, None).await?;

    if response.status().is_success() {
        let boundaries: Vec<Boundary> = response.json().await?;
        return Ok(boundaries);
    }

    Err(Error::APIError("Boundary not found".to_string()))
}

pub async fn get_neighbourhood_team(force_id: &str, neighbourhood_id: &str) -> Result<Vec<Officer>, Error> {
    let url = format!("{}/{}/{}/people", BASE_URL, force_id, neighbourhood_id);

    let response = base_request(&url, None).await?;

    if response.status().is_success() {
        let team: Vec<Officer> = response.json().await?;
        return Ok(team);
    }

    Err(Error::APIError("Team not found".to_string()))
}

pub async fn get_neighbourhood_events(force_id: &str, neighbourhood_id: &str) -> Result<Vec<Event>, Error> {
    let url = format!("{}/{}/{}/events", BASE_URL, force_id, neighbourhood_id);

    let response = base_request(&url, None).await?;

    if response.status().is_success() {
        let events: Vec<Event> = response.json().await?;
        return Ok(events);
    }

    Err(Error::APIError("Events not found".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test(tokio::test)]
    async fn test_list_force_neighbourhoods() {
        let neighbourhoods = list_force_neighbourhoods("leicestershire").await;

        assert!(neighbourhoods.is_ok());

        assert!(neighbourhoods.unwrap().len() > 0);
    }

    #[test(tokio::test)]
    async fn test_get_specific_neighbourhood() {
        let neighbourhood = get_specific_neighbourhood("leicestershire", "NC04").await;

        assert!(neighbourhood.is_ok());
    }

    #[test(tokio::test)]
    async fn test_get_neighbourhood_boundaries() {
        let boundaries = get_neighbourhood_boundaries("leicestershire", "NC04").await;

        assert!(boundaries.is_ok());
    }

    #[test(tokio::test)]
    async fn test_get_neighbourhood_team() {
        let team = get_neighbourhood_team("leicestershire", "NC04").await;

        assert!(team.is_ok());
    }

    #[test(tokio::test)]
    async fn test_get_neighbourhood_events() {
        let events = get_neighbourhood_events("leicestershire", "NC04").await;

        assert!(events.is_ok());
    }
}