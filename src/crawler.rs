use crate::database::DataBaseConnection;
use crate::domain::Domain;
use crate::error::{CrawlError, ErrorType};
use crate::json::UrlsJson;
use crate::parsing::parse_html_links;
use crate::Result;

use rayon::prelude::*;

use reqwest::Url;

use serde_json;

use std::collections::HashSet;
use std::env;
use std::sync::{Arc, Mutex};

// Given a Domain object, tries to crawl its pages starting with the originally requested url.
pub fn crawl(domain: &Domain) -> Result<UrlsJson> {
    let mut db = DataBaseConnection::new()?;
    let limit = env::var("URL_LIST_MAX_SIZE")?.parse().unwrap_or(50);
    let name = domain.get_name();

    // If the cache contains a set of urls for this domain name, returns that set.
    if let Some(url_set) = db.get_set(&name) {
        let json = UrlsJson::new(&name, url_set);
        println!(
            "url set extracted: {}",
            serde_json::to_string(&json).unwrap()
        );
        return Ok(json);
    }

    // Initializes a url queue and a shared set for visited urls
    let url = Url::parse(domain.get_original_url())?;
    let mut url_queue = Vec::new();
    let url_set_pointer = Arc::new(Mutex::new(HashSet::new()));
    url_queue.push(url);

    // Parallel threads are responsible to fetch the content of each url, mark it
    // as visited, parse its links and return them for another round of crawling.
    while !url_queue.is_empty() {
        url_queue = url_queue
            .into_par_iter()
            .map_with(url_set_pointer.clone(), |set, url| {
                if set.lock().unwrap().len() >= limit {
                    vec![]
                } else if !domain.is_in_domain(&url) {
                    println!("Outside the domain: {}", url);
                    vec![]
                } else if set.lock().unwrap().contains(url.as_str()) {
                    println!("Already in domain: {}", url);
                    vec![]
                } else {
                    println!("Adding: {}", url);
                    set.lock().unwrap().insert(url.as_str().to_owned());
                    match fetch(url) {
                        Ok(html) => parse_html_links(domain, html),
                        Err(_) => vec![],
                    }
                }
            })
            .flatten()
            .collect();
    }

    // Takes the set out its shared structure.
    let url_set = Arc::try_unwrap(url_set_pointer)
        .map_err(|_| CrawlError::new(ErrorType::ScrapError))?
        .into_inner()
        .map_err(|_| CrawlError::new(ErrorType::ScrapError))?;

    db.set(&name, url_set.clone())?;
    let json = UrlsJson::new(&name, url_set);
    Ok(json)
}

// Gets the html content of a page.
fn fetch(link: Url) -> Result<String> {
    reqwest::get(link)
        .and_then(|mut resp| resp.text())
        .map_err(|_| CrawlError::new(ErrorType::FetchError))
}
