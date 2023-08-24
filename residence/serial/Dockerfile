FROM curlimages/curl:latest AS downloader
WORKDIR /download
RUN curl -fsSL https://raw.githubusercontent.com/arduino/arduino-cli/master/install.sh | sh

FROM rust:latest
WORKDIR /app
COPY . .
COPY --from=downloader /download/bin/arduino-cli /usr/local/bin
RUN cargo build --release
CMD ["./target/release/residence-serial"]
