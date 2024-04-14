use serde::{Deserialize, Serialize};
use crate::models::contact_details::ContactDetails;


#[derive(Serialize, Deserialize)]
pub struct Officer {
    /// Senior officer biography
    pub bio: Option<String>,
    /// Contact details for officer
    pub contact_details: ContactDetails,
    /// Name of the officer
    pub name: String,
    /// Force Rank
    pub rank: Option<String>, // wales doesnt specify rank
}
