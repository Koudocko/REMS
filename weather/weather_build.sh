#!/bin/sh

# Install pimoroni project
install_pimoroni(){
  # APT packages install
  sudo apt-get update
  sudo apt-get install -y raspi-config
  
  # I2C bus enable
  sudo raspi-config nonint do_i2c 0
  
  # Build docker image
  sudo docker build -t weather-pimoroni pimoroni
  
  # Run Pimoroni RPi interfacer container
  sudo docker create \
      --privileged \
      --name weather-pimoroni \
      -v /rems/readings/weather.json:/rems/readings/weather.json \
      -v /rems/files/pimoroni/.env:/app/.env \
      -v /dev:/dev \
      weather-pimoroni

  # Obtain webserver IP as well as credentials such as username/password (can be edited in .env file)
  read -p "Host (IP):" HOST
  read -p "User:" USER
  read -p "Password:" PASS
  echo "HOST=$HOST" > /rems/files/pimoroni/.env
  echo "USER=$USER" >> /rems/files/pimoroni/.env
  echo "PASS=$PASS" >> /rems/files/pimoroni/.env
}

# Install webserver project
install_webserver(){
  # Build docker image
  sudo docker build -t weather-webserver webserver

  # Run Django webserver container
  sudo docker create \
      --name weather-webserver \
      -p 8080:8080 \
      -v /rems/readings/weather.json:/rems/readings/weather.json \
      weather-webserver
}

# APT depency check
sudo apt-get update
sudo apt-get install ca-certificates curl gnupg

# Add docker repo to keyring
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/raspbian/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpgsudo 
sudo chmod a+r /etc/apt/keyrings/docker.gpg
echo \
  "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/raspbian \
  "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# Install docker and dependencies with APT
sudo apt-get update
sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# Enable docker service
sudo systemctl enable --now docker

### Install Files
sudo mkdir -p /rems/readings
sudo touch /rems/readings/weather.json
sudo chmod -R 777 /rems/readings

### Install program files
sudo mkdir /rems/files/pimoroni
sudo cp pimoroni/.env /rems/files/pimoroni
sudo chmod -R 777 /rems/files

while true; do
  read -p "Install Pimoroni (1), Webserver (2), or both (3)? " CHOICE

  if [ "$CHOICE" = "1" ]; then
      install_pimoroni
      break
  elif [ "$CHOICE" = "2" ]; then
      install_webserver
      break
  elif [ "$CHOICE" = "3" ]; then
      install_webserver
      install_pimoroni
      break
  else
      echo "Invalid choice!!!"
  fi
done

# Initialze systemd units for startup on boot
sudo cp ./*/*.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now weather-pimoroni
sudo systemctl enable --now weather-webserver
