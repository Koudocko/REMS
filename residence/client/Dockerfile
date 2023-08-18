FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release --manifest-path=client/Cargo.toml
CMD ["./client/target/release/residence-client"]
