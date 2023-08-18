FROM debian:latest
WORKDIR /app
COPY . .
RUN curl -fsSL https://raw.githubusercontent.com/arduino/arduino-cli/master/install.sh | BINDIR=/usr/bin sh
RUN arduino-cli core update-index
RUN arduino-cli core install arduino:avr
RUN arduino-cli lib install OneWire
RUN arduino-cli lib install DallasTemperature
RUN arduino-cli lib install "DHT sensor library"
RUN arduino-cli lib install "Adafruit Unified Sensor"
RUN arduino compile --fqbn arduino:avr:mega 
CMD ["arduino", "compile", "-p", "/dev/ttyACM0", "--fqbn", "arduino:avr:mega"]
