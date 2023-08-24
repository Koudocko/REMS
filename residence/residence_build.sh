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
sudo touch /rems/readings/residence.json
sudo chmod -R 777 /rems/readings

### Docker containers
# Create pull/build images
sudo docker build -t residence-client client 
sudo docker build -t residence-arduino arduino 
sudo docker build -t residence-serial serial 

# Create docker containers from images
# Rust TCP client
sudo docker create \
    --name residence-client \
    -v $(pwd)/client/.env:/app/.env \
    -v /rems/readings/residence.json:/rems/readings/residence.json \
    --restart unless-stopped \
    residence-client

# Arduino program uploader
sudo docker run \
    --name residence-arduino \
    --device=/dev/ttyACM0:/dev/ttyACM0 \
    -d \
    residence-arduino

# Serial port parser
sudo docker create \
    --name residence-serial \
    --device=/dev/ttyACM0:/dev/ttyACM0 \
    -v /rems/readings/residence.json:/rems/readings/residence.json \
    --restart unless-stopped \
    residence-serial

# Obtain residence id and BBP socket (can be edited in .env file)
read -p "Input Residence ID (one word, no symbols except _ underscore): " RESIDENCE_ID
read -p "Input BBP local server socket (IP:PORT): " SERVER_SOCKET
echo "SERVER_SOCKET=$SERVER_SOCKET" > client/.env
echo "RESIDENCE_ID=$RESIDENCE_ID" >> client/.env

# Initialze systemd units for startup on boot
sudo cp client/residence-client.service /etc/systemd/system/
sudo cp serial/residence-serial.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now residence-client
sudo systemctl enable --now residence-serial
