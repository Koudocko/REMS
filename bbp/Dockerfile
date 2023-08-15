FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release
EXPOSE 7879
CMD ["./server/target/release/rems"]
