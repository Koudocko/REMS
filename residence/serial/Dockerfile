FROM curlimages/curl:latest AS downloader
WORKDIR /download
RUN curl -fsSL https://raw.githubusercontent.com/arduino/arduino-cli/master/install.sh | sh

FROM rust:latest AS builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:latest
COPY --from=downloader /download/bin/arduino-cli /usr/local/bin
COPY --from=builder /build/target/release/residence-serial /usr/local/bin
RUN apt-get update && apt-get install -y ca-certificates
RUN arduino-cli core update-index
RUN arduino-cli core install arduino:avr
CMD ["residence-serial"]
