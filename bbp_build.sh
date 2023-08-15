##!/bin/sh

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

# Docker containers
sudo docker pull prom/prometheus
sudo docker run \
    -d \
    -p 7878:7878 \
    -v prometheus.yml:/etc/prometheus/prometheus.yml \
    prom/prometheus
sudo docker pull grafana/grafana
sudo docker run -d --name=grafana -p 3000:3000 grafana/grafana

# Rustup toolchain
curl https://sh.rustup.rs -sSf | sh
source ~/.bashrc

# Install files
sudo mkdir -p /rems/logs
sudo touch /rems/readings/log.txt
sudo chmod -R 777 /rems/readings
