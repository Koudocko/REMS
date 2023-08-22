#!/bin/sh

### APT packages install
sudo apt-get update
sudo apt-get install -y raspi-config

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

# I2C bus enable
sudo raspi-config nonint do_i2c 0

### Install Files
sudo mkdir -p /rems/readings
sudo mkdir -p /rems/commands
sudo touch /rems/readings/weather.json
sudo chmod -R 777 /rems/readings
sudo cp weather.py /rems/commands/weather.py

### Docker containers
# Create docker containers from images
sudo docker build -t weather-pimoroni pimoroni
sudo docker build -t weather-webserver webserver

# Create docker containers from images
# Pimoroni RPi interfacer
sudo docker run \
    --privileged
    --name weather-pimoroni \
    -dv /rems/readings/weather.json:/rems/readings/weather.json \
    -v /dev:/dev \
    weather-pimoroni

# Django webserver 
sudo docker run \
    --name weather-webserver \
    -dp 8080:8080 \
    -v /rems/readings/weather.json:/rems/readings/weather.json \
    weather-webserver

# Obtain webserver IP as well as credentials such as username/password (can be edited in .env file)
read -p "Host (IP):" HOST
read -p "User:" USER
read -p "Password:" PASS
echo -e "HOST=$HOST\nUSER=$USER\nPASS:$PASS" > .env
