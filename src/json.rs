use serde::Serialize;

use std::collections::HashSet;
use std::fmt;

// JSON formatted object that is responsible for the transformation of a urls set into the response body.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UrlsJson {
	pub nb_urls: usize,
	pub domain_crawled: String,
	pub urls: Vec<String>,
}

impl UrlsJson {
	pub fn new(domain_name: &str, set: HashSet<String>) -> Self {
		UrlsJson {
			nb_urls: set.len(),
			domain_crawled: domain_name.to_owned(),
			urls: set.into_iter().collect(),
		}
	}
}

impl fmt::Display for UrlsJson {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let message = serde_json::to_string_pretty(self).unwrap_or_else(|_| {
			"An error occured while serializing the answer to json format".to_owned()
		});
		write!(fmt, "{}", message)
	}
}

// JSON format for the nb-urls response.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NbJson {
	pub nb_urls: usize,
	pub domain_crawled: String,
}

impl fmt::Display for NbJson {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let message = serde_json::to_string_pretty(self).unwrap_or_else(|_| {
			"An error occured while serializing the answer to json format".to_owned()
		});
		write!(fmt, "{}", message)
	}
}

// JSON format for the CrawlError structs.
#[derive(Serialize, Debug)]
pub struct ErrorJson {
	pub error: String,
}
