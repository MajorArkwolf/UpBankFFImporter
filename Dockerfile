FROM rust:1-alpine
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./LICENSE.md ./LICENSE.md
RUN apk add musl-dev musl
RUN cargo install -v --path .
ENTRYPOINT "up_bank_fidi"
