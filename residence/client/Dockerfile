FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["./client/target/release/residence-client"]
