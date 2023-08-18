FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release --manifest-path=server/Cargo.toml
EXPOSE 7879
CMD ["./server/target/release/bbp-server"]
