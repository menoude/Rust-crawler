use crate::crawler::crawl;
use crate::database::DataBaseConnection;
use crate::domain::Domain;
use crate::error::{CrawlError, ErrorType};
use crate::json::{NbJson, UrlsJson};
use crate::parsing;
use crate::Result;

use futures::{future, future::Either, Future};

use hyper::rt::run;
use hyper::rt::Stream;
use hyper::service::service_fn;
use hyper::{body::Body, Method, Request, Response, Server, StatusCode};

use std::net::SocketAddr;
use std::str::from_utf8;
use std::string::ToString;

// Starts the server, panics in case of error.
pub fn start_server(addr: SocketAddr) {
    let service = || service_fn(routing);

    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| panic!(format!("Server error: {}", e)));
    println!("Listening to {}", addr);
    run(server);
}

// Dispatches the requests according to their methods and routes.
fn routing(req: Request<Body>) -> impl Future<Item = Response<Body>, Error = hyper::Error> {
    let resp = match (req.method(), req.uri().path()) {
        (&Method::GET, "/urls") => handle_list(req),
        (&Method::GET, "/nb-urls") => handle_nb(req),
        (&Method::POST, "/crawl") => {
            return Either::A(req.into_body().concat2().map(|content| {
                match from_utf8(&content) {
                    Ok(data) => handle_crawl(data),
                    Err(e) => Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body(Body::from(format!("error: {}", e)))
                        .unwrap(),
                }
            }))
        }
        (method, path) => handle_other_requests(method, path),
    };
    Either::B(future::ok(resp))
}

// Creates a Domain object from the post data, and tries to crawl the corresponding domain.
fn handle_crawl(content: &str) -> Response<Body> {
    let result =
        Domain::new(&content).and_then(|domain| crawl(&domain).map(|json| json.to_string()));
    send_ok_or_err(result)
}

// Creates a Domain object from the query's domain parameter, and looks for a domain in the database.
// Returns its urls.
fn handle_list(req: Request<Body>) -> Response<Body> {
    let result = parsing::parse_domain(req.uri()).and_then(|name| {
        DataBaseConnection::new().and_then(|ref mut db| {
            db.get_set(&name)
                .ok_or_else(|| CrawlError::new(ErrorType::DomainNotCrawled))
                .map(|set| UrlsJson::new(&name, set).to_string())
        })
    });
    send_ok_or_err(result)
}

// Creates a Domain object from the query's domain parameter, and looks for the number of urls
// stored for that domain in the database.
fn handle_nb(req: Request<Body>) -> Response<Body> {
    let result = parsing::parse_domain(req.uri()).and_then(|name| {
        DataBaseConnection::new().and_then(|ref mut db| {
            println!("name of the parameter: {}", name);
            db.get_len(&name)
                .ok_or_else(|| CrawlError::new(ErrorType::DomainNotCrawled))
                .map(|(name, len)| {
                    NbJson {
                        nb_urls: len,
                        domain_crawled: name.to_owned(),
                    }
                    .to_string()
                })
        })
    });
    send_ok_or_err(result)
}

// Returns an error response for wrong paths / methods.
fn handle_other_requests(method: &Method, path: &str) -> Response<Body> {
    let message = format!("{} {} is not supported.\r\n", method.as_str(), path);
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(message))
        .unwrap()
}

// Maps a result from one of the endpoint functions to an HTTP response.
fn send_ok_or_err(result: Result<String>) -> Response<Body> {
    match result {
        Ok(set) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(format!("{}\r\n", set)))
            .unwrap(),
        Err(CrawlError { kind, code }) => Response::builder()
            .status(code)
            .body(Body::from(format!("{}\r\n", kind)))
            .unwrap(),
    }
}
