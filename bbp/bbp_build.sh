#!/bin/sh

### Docker
# APT dependency check
sudo apt-get update
sudo apt-get install -y ca-certificates curl gnupg python3-pip python3 python-is-python3

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

# Add systemd service gui monitor
sudo pip install django
echo "bbp-server bbp-grafana bbp-prometheus" >> ../monitor/monitor/static/services/services.txt
sudo cp -r ../monitor /rems/files/

### Install log file and directory
sudo mkdir -p /rems/logs
sudo touch /rems/logs/log.txt
sudo chmod -R 777 /rems/logs

### Install program files
sudo mkdir /rems/files/server
sudo cp server/.env /rems/files/server
sudo mkdir /rems/files/prometheus
sudo cp prometheus/prometheus.yml /rems/files/prometheus
sudo mkdir /rems/files/grafana
sudo cp grafana/datasource.yml /rems/files/grafana
sudo chmod -R 777 /rems/files

### Docker containers
# Create network and pull/build images
sudo docker network create bbp
sudo docker pull prom/prometheus
sudo docker pull grafana/grafana
sudo docker build -t bbp-server server 

# Create docker containers from images
# Rust TCP server / general handler 
sudo docker create \
    --network bbp \
    --name bbp-server \
    -p 7879:7879 \
    -v /rems/files/server/.env:/app/.env \
    -v /rems/logs/log.txt:/rems/logs/log.txt \
    bbp-server

# Prometheus server
sudo docker create \
    --network bbp \
    --name bbp-prometheus \
    -p 7878:7878 \
    -v /rems/files/prometheus/prometheus.yml:/etc/prometheus/prometheus.yml \
    prom/prometheus

# Grafana service
sudo docker create \
    --network bbp \
    --name=bbp-grafana \
    -p 3000:3000 \
    -v /rems/files/grafana/datasource.yml:/etc/grafana/provisioning/datasources/datasource.yml \
    grafana/grafana

# Obtain grafana API token from user via cli (can be edited in .env file) 
read -p "Create service account with admin, add token, and enter Grafana API token (localhost:3000 > Administration > Service Accounts): " API_TOKEN
echo "API_TOKEN=$API_TOKEN" > /rems/files/server/.env

# Restart server to use updated token
sudo docker restart bbp-server

# Initialze systemd units for startup on boot
sudo cp ./*/*.service /etc/systemd/system/
sudo cp /rems/files/monitor/service-monitor.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now bbp-server
sudo systemctl enable --now bbp-grafana
sudo systemctl enable --now bbp-prometheus
sudo systemctl enable --now service-monitor
