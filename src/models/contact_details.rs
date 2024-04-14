use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactDetails {
    /// Email address
    pub email: Option<String>,
    /// Telephone number
    pub telephone: Option<String>,
    /// Mobile number
    pub mobile: Option<String>,
    /// Fax number
    pub fax: Option<String>,
    /// Website address
    pub web: Option<String>,
    /// Street address
    pub address: Option<String>,
    /// Facebook profile URL
    pub facebook: Option<String>,
    /// Twitter profile URL
    pub twitter: Option<String>,
    /// YouTube profile URL
    pub youtube: Option<String>,
    /// Myspace profile URL
    pub myspace: Option<String>,
    /// Bebo profile URL
    pub bebo: Option<String>,
    /// Flickr profile URL
    pub flickr: Option<String>,
    /// Google+ profile URL
    pub google_plus: Option<String>,
    /// Forum URL
    pub forum: Option<String>,
    /// E-messaging URL
    pub e_messaging: Option<String>,
    /// Blog URL
    pub blog: Option<String>,
    /// RSS URL
    pub rss: Option<String>,
}


impl ContactDetails {
    pub fn has_any(&self) -> bool {
        self.email.is_some() || self.telephone.is_some() || self.mobile.is_some() || self.fax.is_some() || self.web.is_some() || self.address.is_some() || self.facebook.is_some() || self.twitter.is_some() || self.youtube.is_some() || self.myspace.is_some() || self.bebo.is_some() || self.flickr.is_some() || self.google_plus.is_some() || self.forum.is_some() || self.e_messaging.is_some() || self.blog.is_some() || self.rss.is_some()
    }
}