use crate::domain::Domain;
use crate::error::{CrawlError, ErrorType};
use crate::Result;

use hyper::Uri;

use scraper::{Html, Selector};

use url::{form_urlencoded, Url};

// Returns a valid domain as a string, parsing it from a GET request query parameter.
pub fn parse_domain(uri: &Uri) -> Result<String> {

    // Gets the 'domain' parameter from the query.
    let (_, domain_parameter) = uri
        .query()
        .and_then(|query| {
            form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .find(|pair| pair.0 == "domain")
        })
        .ok_or_else(|| CrawlError::new(ErrorType::MissingParameter))?;

    // Checks that it is a well formatted domain and returns it as a String.
    Url::parse(&domain_parameter)?
        .domain()
        .ok_or_else(|| CrawlError::new(ErrorType::WrongDomain))
        .map(ToOwned::to_owned)

}

// Returns a vector of reqwest::Url objects containing every hyperlink found in some HTML string.
pub fn parse_html_links(domain: &Domain, html: String) -> Vec<reqwest::Url> {
    // Parses the HTML.
    let dom = Html::parse_document(&html);
    let link_selector = Selector::parse("a").unwrap();
    let mut links = vec![];

    // Iterates through each hyperlink.
    for element in dom.select(&link_selector) {
        if let Some(href) = element.value().attr("href") {
            // Checks if the link is relative, and completes it with the domain name if necessary
            if href.starts_with('/') {
                links.push(domain.create_url_from_path(href));
            } else if let Ok(url) = Url::parse(href) {
                links.push(url);
            }
        }
    }
    links
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_domain_parsing() {
        let valid_uri_query =
            Uri::from_str("https://www.webcrawler.com/nb-urls?domain=https://docs.rs").unwrap();
        let missing_parameter_uri = Uri::from_str("https://www.webcrawler.com/nb-urls").unwrap();

        assert_eq!(parse_domain(&valid_uri_query), Ok(String::from("docs.rs")));
        assert_eq!(
            parse_domain(&missing_parameter_uri),
            Err(CrawlError::new(ErrorType::MissingParameter))
        );
    }

    #[test]
    fn test_html_parsing() {
        let domain = Domain::new("https://docs.rs").unwrap();
        let html = r##"
            <a href="#">
            <a href="/someotherdocumentation">
            <p>Nothing special</p>
            <a href="https://docs.rs/hyper/0.12.32/hyper/">
            "##
        .to_owned();
        let mut links = parse_html_links(&domain, html).into_iter();
        assert!(links.next().unwrap().has_host());
        assert_eq!(
            links.next().unwrap(),
            Url::parse("https://docs.rs/hyper/0.12.32/hyper/").unwrap()
        )
    }
}
