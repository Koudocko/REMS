FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release --manifest-path=server/Cargo.toml
CMD ["./client/target/release/residence-client"]
