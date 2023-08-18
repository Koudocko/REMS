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
sudo mkdir -p /rems/logs
sudo touch /rems/logs/log.txt
sudo chmod -R 777 /rems/logs

# Docker containers
sudo docker network create bbp
sudo docker pull prom/prometheus
sudo docker pull grafana/grafana
sudo docker build -t bbp-server . 
sudo docker run \
    --network bbp \
    --name bbp-server \
    -dp 7879:7879 \
    -v $(pwd)/.env:/app/.env \
    -v /rems/logs/log.txt:/rems/logs/log.txt \
    bbp-server
sudo docker run \
    --network bbp \
    --name bbp-prometheus \
    -dp 7878:7878 \
    -v $(pwd)/prometheus.yml:/etc/prometheus/prometheus.yml \
    prom/prometheus
sudo docker run \
    --network bbp \
    --name=bbp-grafana \
    -dp 3000:3000 \
    -v $(pwd)/datasource.yml:/etc/grafana/provisioning/datasources/datasource.yml \
    grafana/grafana
read -p "Create service account with admin, add token, and enter Grafana API token (localhost:3000 > Administration > Service Accounts): " API_TOKEN
echo "API_TOKEN=$API_TOKEN" > .env
sudo docker restart bbp-server

