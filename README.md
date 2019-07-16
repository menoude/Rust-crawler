# web-crawler

A Rust web crawler server.

## Requirements

Either Rust and Redis, or Docker.

### How to use

#### Manually

- Modify `.env` file at the root of the project with your host address, port, the address of your Redis server and your max number of urls to crawl. Example:

```
HOST_ADDRESS="0.0.0.0"
HOST_PORT="3000"
DATABASE_URL="redis://127.0.0.1/"
URL_LIST_MAX_SIZE="50"
```

- Launch your Redis server
- `Cargo run`

#### With Docker and docker-compose

- The defaut port is 3000 on `localhost`, but you can modify it through the first port in `services.server.ports` in `docker-compose.yml`. You can also modify the max number of urls crawled per domain by modifying `services.server.environment.URL_LIST_MAX_SIZE`.
- `docker-compose up`

There won't be any guaranteed persistence of your database with that method.
The crawling speed should be slower when running this app in a container (probably because of multi-threading management)

### Endpoints

`POST /crawl {url}`

The payload should be a valid url.
Crawls the domain corresponding to the url in the payload, starting from that url. Returns the result as a JSON object.

`GET /urls?domain={url}`

The parameter should be a valid and complete url, url-encoded.
Looks-up in the database for the presence of domain previously crawled. Returns the result as a JSON object.

`GET /nb-urls?domain{url}`

The parameter should be a valid and complete url, url-encoded.
Looks-up in the database for the presence of domain previously crawled. Returns the number of urls crawled for the domain in a JSON object.
