#!/bin/sh

# Docker
sudo apt-get update
sudo apt-get install ca-certificates curl gnupg
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
sudo chmod a+r /etc/apt/keyrings/docker.gpg
echo \
  "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo systemctl enable --now docker

# Install files
sudo mkdir -p /rems/readings
sudo touch /rems/readings/residence.txt
sudo chmod -R 777 /rems/residence

# Docker containers
sudo docker build -t residence-client client 
sudo docker build -t residence-arduino arduino 
sudo docker build -t residence-serial serial 
sudo docker run \
    --name residence-client \
    -dv $(pwd)/client/.env:/app/.env \
    -v /rems/readings/residence.txt:/rems/readings/residence.txt \
    residence-client
sudo docker run \
    --name residence-arduino \
    --device=/dev/ttyACM0:/dev/ttyACM0 \
    -d \
    residence-arduino
sudo docker run \
    --name residence-serial \
    --device=/dev/ttyACM0:/dev/ttyACM0 \
    -dv /rems/readings/residence.txt:/rems/readings/residence.txt \
    residence-serial
read -p "Input Residence ID (one word, no symbols except _ underscore): " RESIDENCE_ID
read -p "Input BBP local server socket (IP:PORT): " SERVER_SOCKET
echo -e "SERVER_SOCKET=$SERVER_SOCKET\nRESIDENCE_ID=$RESIDENCE_ID" > .env
