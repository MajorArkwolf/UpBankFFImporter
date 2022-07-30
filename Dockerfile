FROM rust
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./LICENSE.md ./LICENSE.md
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
ENTRYPOINT "./target/release/up_bank_fidi"
