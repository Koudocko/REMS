FROM debian:latest AS downloader
WORKDIR /download
RUN curl -fsSL https://raw.githubusercontent.com/arduino/arduino-cli/master/install.sh | sh

FROM debian:latest
WORKDIR /app
COPY . .
COPY --from=downloader /download/bin/arduino-cli /usr/local/bin/arduino-cli
RUN arduino-cli core update-index
RUN arduino-cli core install arduino:avr
RUN arduino-cli lib install OneWire
RUN arduino-cli lib install DallasTemperature
RUN arduino-cli lib install "DHT sensor library"
RUN arduino-cli lib install "Adafruit Unified Sensor"
RUN arduino-cli compile --fqbn arduino:avr:mega 
CMD ["arduino", "compile", "-p", "/dev/ttyACM0", "--fqbn", "arduino:avr:mega"]
