use chrono::NaiveDate;
use crate::BASE_URL;
use crate::requests::base_request;
use crate::errors::Error;
use crate::models::crime_category::Category;

async fn get_crime_categories(date: NaiveDate) -> Result<Vec<Category>, Error> {
    let url = format!("{}/crime-categories", BASE_URL);

    let date_strf = date.format("%Y-%m").to_string();

    let response = base_request(&url, Some(vec![("date", date_strf.as_str())])).await?;

    if response.status().is_success() {
        let categories: Vec<Category> = response.json().await?;
        return Ok(categories);
    }

    Err(Error::APIError("No crime categories found".to_string()))
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test(tokio::test)]
    async fn test_get_crime_categories() {
        let date = NaiveDate::from_ymd(2021, 1, 1);
        let crime_categories = get_crime_categories(date).await;

        assert!(crime_categories.is_ok());
    }
}