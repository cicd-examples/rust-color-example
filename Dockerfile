FROM rust:1.58-slim

WORKDIR /usr/src/rust-color-example
COPY . .

RUN cargo install --path .

CMD ["rust-color-example"]
