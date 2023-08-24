#!/bin/sh

### Docker
# APT dependency check
sudo apt-get update
sudo apt-get install -y ca-certificates curl gnupg

# Add docker repo to keyring
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
sudo chmod a+r /etc/apt/keyrings/docker.gpg
echo \
  "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# Install docker and dependencies with APT
sudo apt-get update
sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# Enable docker service
sudo systemctl enable --now docker

# Install reading file and directory
sudo mkdir -p /rems/readings
sudo mkdir -p /rems/files
sudo touch /rems/readings/residence.json
sudo chmod -R 777 /rems/readings

### Install program files
sudo cp -R arduino /rems/files
sudo rm /rems/files/arduino/arduino-upload
sudo mkdir client
sudo cp client/.env /rems/files/client
sudo chmod -R 777 /rems/files

### Install arduino-cli and libraries
curl -fsSL https://raw.githubusercontent.com/arduino/arduino-cli/master/install.sh | BINDIR=~ sh 
sudo mv arduino-cli /usr/local/bin/arduino-cli
sudo cp arduino/arduino-upload /usr/local/bin
arduino-cli core update-index
arduino-cli core install arduino:avr
arduino-cli lib install OneWire
arduino-cli lib install DallasTemperature
arduino-cli lib install "DHT sensor library"
arduino-cli lib install "Adafruit Unified Sensor"
arduino-upload

### Docker containers
# Create pull/build images
sudo docker build -t residence-client client 
sudo docker build -t residence-serial serial 

# Create docker containers from images
# Rust TCP client
sudo docker create \
    --name residence-client \
    -v /rems/files/client/.env:/app/.env \
    -v /rems/readings/residence.json:/rems/readings/residence.json \
    residence-client

# Serial port parser
sudo docker create \
    --name residence-serial \
    --device=/dev/ttyACM0:/dev/ttyACM0 \
    -v /rems/readings/residence.json:/rems/readings/residence.json \
    residence-serial

# Obtain residence id and BBP socket (can be edited in .env file)
read -p "Input Residence ID (one word, no symbols except _ underscore): " RESIDENCE_ID
read -p "Input BBP local server IP (IP): " SERVER_IP
echo "SERVER_IP=$SERVER_IP:7879" > /rems/files/client/.env
echo "RESIDENCE_ID=$RESIDENCE_ID" >> /rems/files/client/.env

# Initialze systemd units for startup on boot
sudo cp client/residence-client.service /etc/systemd/system/
sudo cp serial/residence-serial.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now residence-client
sudo systemctl enable --now residence-serial
