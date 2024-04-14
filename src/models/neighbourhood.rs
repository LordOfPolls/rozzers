use serde::{Deserialize, Serialize};
use crate::models::contact_details::ContactDetails;
use crate::models::link::Link;
use crate::models::location::Location;
use crate::requests::get_specific_neighbourhood;
use crate::errors::Error;

#[derive(Serialize, Deserialize)]
struct Centre {
    pub latitude: String,
    pub longitude: String,
}

#[derive(Serialize, Deserialize)]
pub struct Neighbourhood {
    /// Police force specific team identifier.
    /// Note: this identifier is not unique and may also be used by a different force
    pub id: String,
    /// Name for the neighbourhood
    pub name: String,
    /// Force identifier
    #[serde(skip)]
    pub force_id: String,

    /// URL for the neighbourhood on the Force's website
    #[serde(default)]
    pub url_force: Option<String>,
    /// Ways to get in touch with the neighbourhood officers
    #[serde(default)]
    pub contact_details: ContactDetails,
    /// Links to other resources
    #[serde(default)]
    pub links: Vec<Link>,
    /// Centre point locator for the neighbourhood.
    /// Note: This may not be exactly in the centre of the neighbourhood
    #[serde(default)]
    pub centre: Option<Centre>,
    /// Any associated locations with the neighbourhood, e.g. police stations
    #[serde(default)]
    pub locations: Vec<Location>,
    /// Description of the neighbourhood
    #[serde(default)]
    pub description: Option<String>,
    /// Population of the neighbourhood
    #[serde(default)]
    pub population: Option<String>,
}

impl Neighbourhood {
    pub async fn get_detailed(&self) -> Result<Neighbourhood, Error> {
        get_specific_neighbourhood(&self.force_id, &self.id).await
    }

    pub async fn get_boundaries(&self) -> Result<Vec<Boundary>, Error> {
        let boundaries = crate::requests::get_neighbourhood_boundaries(&self.force_id, &self.id).await;
        boundaries
    }

    pub async fn get_team(&self) -> Result<Vec<crate::models::officer::Officer>, Error> {
        crate::requests::get_neighbourhood_team(&self.force_id, &self.id).await
    }
}

#[derive(Serialize, Deserialize)]
pub struct Boundary {
    pub latitude: String,
    pub longitude: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::requests::list_all_forces;
    use test_log::test;

    #[test(tokio::test)]
    async fn test_helper_methods() {
        let forces = list_all_forces().await;

        assert!(forces.is_ok());

        let binding = forces.unwrap();
        for force in binding {
            let neighbourhoods = force.get_neighbourhoods().await;
            assert!(neighbourhoods.is_ok());

            let binding = neighbourhoods.unwrap();
            let detailed = binding[0].get_detailed().await;

            assert!(detailed.is_ok());

            let boundaries = binding[0].get_boundaries().await;
            assert!(boundaries.is_ok());

            let team = binding[0].get_team().await;
            assert!(team.is_ok());
        }
    }
}