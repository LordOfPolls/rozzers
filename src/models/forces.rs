use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use crate::models::engagement::EngagementMethod;
use crate::models::officer::Officer;
use crate::models::neighbourhood::Neighbourhood;
use crate::models::availability::Availability;
use crate::errors::Error;
use crate::requests::{get_specific_force, get_force_senior_officers, get_detailed_availability, list_force_neighbourhoods};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Force {
    /// Unique force identifier
    pub id: String,
    /// Force name
    pub name: String,

    /// Description
    #[serde(default)]
    pub description: Option<String>,
    /// Force website URL
    #[serde(default)]
    pub url: Option<String>,
    /// Ways to keep informed
    #[serde(default)]
    pub engagement_methods: Vec<EngagementMethod>,
    /// Force telephone number
    #[serde(default)]
    pub telephone: Option<String>,
}

impl Default for Force {
    fn default() -> Self {
        Force {
            id: "".to_string(),
            name: "".to_string(),
            description: None,
            url: None,
            engagement_methods: vec![],
            telephone: None,
        }
    }
}

impl Force {
    pub async fn get_detailed(&self) -> Result<Force, Error> {
        get_specific_force(&self.id).await
    }

    pub async fn get_senior_officers(&self) -> Result<Vec<Officer>, Error> {
        get_force_senior_officers(&self.id).await
    }

    pub async fn get_neighbourhoods(&self) -> Result<Vec<Neighbourhood>, Error> {
        list_force_neighbourhoods(&self.id).await
    }

    pub async fn get_stop_and_search_availability(&self) -> Result<Vec<NaiveDate>, Error> {
        let availability = get_detailed_availability().await?;

        let mut available_months = vec![];

        for month in availability {
            if month.forces.contains(&self.id) {
                available_months.push(month.date);
            }
        }

        Ok(available_months)
    }
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
            let force = force.get_detailed().await.unwrap();

            let something_populated = force.description.is_some() || force.url.is_some() || force.telephone.is_some() || force.engagement_methods.len() > 0;
            assert!(something_populated);

            let officers = force.get_senior_officers().await;
            assert!(officers.is_ok());

            let neighbourhoods = force.get_neighbourhoods().await;
            assert!(neighbourhoods.is_ok());

            let availability = force.get_stop_and_search_availability().await;
            assert!(availability.is_ok());
        }
    }


}