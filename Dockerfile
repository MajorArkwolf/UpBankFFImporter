FROM rust:1-alpine
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./LICENSE.md ./LICENSE.md
COPY ./config/settings-template.yaml ./config/settings-template.yaml
RUN apk add musl-dev musl
RUN cargo install -v --path .
ENTRYPOINT "up_bank_fidi"
