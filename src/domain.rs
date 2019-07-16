use crate::error::{CrawlError, ErrorType};
use crate::Result;

use reqwest::Url;

// Struct representing a domain with it base address and the originally requested url.
#[derive(Debug, PartialEq)]
pub struct Domain {
    domain_name: String,
    original_url: Url,
}

impl Domain {
    // Checks that the originally requested url is valid, creates a Domain object.
    pub fn new(candidate_url: &str) -> Result<Self> {
        let url = Url::parse(candidate_url)?;
        let domain = url
            .domain()
            .ok_or_else(|| CrawlError::new(ErrorType::WrongDomain))?
            .to_owned();
        Ok(Domain {
            original_url: url,
            domain_name: domain,
        })
    }

    pub fn get_name(&self) -> String {
        self.domain_name.clone()
    }

    pub fn get_original_url(&self) -> &str {
        self.original_url.as_str()
    }

    // Completes a relative url with the base address of that domain.
    pub fn create_url_from_path(&self, path: &str) -> Url {
        let mut new_url = self.original_url.clone();
        new_url.set_path(path);
        new_url
    }

    // Checks that the domain addresses correspond to each other.
    pub fn is_in_domain(&self, link: &Url) -> bool {
        match link.domain() {
            Some(domain) => self.domain_name == domain,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_domain() {
        let candidate_url = "https://www.theguardian.com/international";
        let comparison_struct = Domain {
            domain_name: String::from("www.theguardian.com"),
            original_url: Url::parse("https://www.theguardian.com/international").unwrap(),
        };
        assert_eq!(Domain::new(candidate_url), Ok(comparison_struct));
        let invalid_url = "htp:/theguardian..com";
        assert_eq!(
            Domain::new(invalid_url),
            Err(CrawlError::new(ErrorType::WrongDomain))
        );
    }

    #[test]
    fn test_domain_api() {
        let domain = Domain::new("https://www.theguardian.com/international").unwrap();
        assert_eq!(domain.get_name(), "www.theguardian.com");
        assert_eq!(
            domain.get_original_url(),
            "https://www.theguardian.com/international"
        );

        let in_domain = Url::parse("https://www.theguardian.com/uk/lifeandstyle").unwrap();
        assert!(domain.is_in_domain(&in_domain));
        let other_domain = Url::parse("https://www.nytimes.com/section/world").unwrap();
        assert!(!domain.is_in_domain(&other_domain));

        assert_eq!(
            domain.create_url_from_path("uk/sports"),
            Url::parse("https://www.theguardian.com/uk/sports").unwrap()
        );
    }
}
