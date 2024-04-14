use std::time::Duration;
use log::{debug, error, info, warn};
use once_cell::sync::Lazy;
use reqwest::RequestBuilder;

pub use availability::*;
pub use forces::*;
pub use neighbourhoods::*;

use crate::errors::Error;
use crate::models::ratelimit::LeakyBucket;

mod forces;
mod neighbourhoods;
mod availability;
mod crimes;

const MAX_RETRIES: u16 = 3;
const RETRY_BACKOFF_SECS: u64 = 2;

// Police API rate limit is 15 requests per second (30 burst)
static RATE_LIMITER : Lazy<LeakyBucket> = Lazy::new(|| LeakyBucket::new(30, 15, Duration::from_millis(67)));


pub async fn base_request(url: &str, params: Option<Vec<(&str, &str)>>) -> Result<reqwest::Response, Error> {
    let client = reqwest::Client::new();

    let mut attempt: u16 = 0;

    loop {
        RATE_LIMITER.acquire().await;
        let mut request: RequestBuilder = client.get(url.trim());

        if let Some(ref params) = params {
            request = request.query(&params);
        }

        let response = match request.send().await {
            Ok(response) => response,
            Err(err) => {
                error!("Request failed: {}", err);
                return Err(Error::ReqwestError(err));
            }
        };

        match response.status().as_u16() {
            200..=299 => {
                info!("Success ({}): Requested {} with params {:?}", response.status(), url, params);
                return Ok(response);
            }
            429 => {
                warn!("Rate Limited ({}): Requested {} with params {:?}", response.status(), url, params);
            }
            400..=499 => {
                error!("Client Error ({}): Requested {} with params {:?}", response.status(), url, params);
                return Err(Error::ClientError(format!("Client error: {}", response.status())));
            }
            500..=599 => {
                warn!("Server Error ({}): Requested {} with params {:?}", response.status(), url, params);
            }
            _ => {
                warn!("Unexpected Status ({}): Requested {} with params {:?}", response.status(), url, params);
            }
        }

        attempt += 1;

        if attempt > MAX_RETRIES {
            error!("Max retries exceeded ({}): Requested {} with params {:?}", response.status(), url, params);
            return Err(Error::MaxRetriesExceeded(format!("Max retries exceeded: {}", response.status())));
        }

        let backoff_duration = tokio::time::Duration::from_secs(RETRY_BACKOFF_SECS * attempt as u64);
        warn!("Retrying request in {} seconds", backoff_duration.as_secs());
        tokio::time::sleep(backoff_duration).await;
    }
}