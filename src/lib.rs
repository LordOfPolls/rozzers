mod models;
mod requests;
mod errors;
mod utils;

const BASE_URL: &str = "https://data.police.uk/api";


#[cfg(test)]
mod tests {
    use super::*;
}
