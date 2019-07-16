FROM rust
WORKDIR /usr/src/crawler
COPY . .
RUN cargo install --path .
EXPOSE 3000
CMD ["crawler"]