mod forces;
mod neighbourhoods;
mod availability;
mod crimes;

pub use forces::*;
pub use neighbourhoods::*;
pub use availability::*;

use log;
use reqwest::RequestBuilder;
use crate::errors::Error;

pub async fn base_request(url: &str, params: Option<Vec<(&str, &str)>>) -> Result<reqwest::Response, Error> {
    let client = reqwest::Client::new();

    let mut attempt: u16 = 0;

    loop {
        let mut request: RequestBuilder = client.get(url.trim());

        if let Some(ref params) = params {
            request = request.query(&params);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            log::info!("200 :: Requested {} with params {:?}", url, params);
            return Ok(response);
        }

        if response.status().is_client_error() {
            log::error!("{} :: Requested {} with params {:?} -- Client error", &response.status(), url, params);
            return Err(Error::ClientError("Client error".to_string()));
        }

        log::warn!("{} :: Requested {} with params {:?} -- Retrying", &response.status(), url, params);

        if attempt >= 3 {
            log::error!("{} :: Requested {} with params {:?} -- Giving up", &response.status(), url, params);
        }
        log::warn!("Retrying request");
        attempt += 1;

        tokio::time::sleep(tokio::time::Duration::from_secs((attempt * attempt) as u64)).await;
    }
}