#!/bin/sh

### APT packages install
sudo apt-get update
sudo apt-get install python3 python3-pip apache2 libapache2-mod-wsgi-py3 python3-django cron
pip install django python-dotenv

### Breakout garden modules install
wget https://github.com/pimoroni/breakout-garden/archive/master.zip
unzip master.zip
sudo ./breakout-garden-master/install.sh
sudo rm -r breakout-garden-master

# Install Files
sudo mkdir -p /rems/readings
sudo mkdir -p /rems/commands
sudo touch /rems/readings/weather.txt
sudo chmod -R 777 /rems/readings
sudo cp weather.py /rems/commands/weather.py
sudo cp transfer.sh /rems/commands/transfer.sh


### Docker containers
# Create docker containers from images
sudo docker pull httpd
sudo docker build -t weather-pimoroni pimoroni
sudo docker build -t weather-webserver webserver

# Create docker containers from images
# Apache web service
sudo docker run \
    --name weather-apache \
    weather-apache

# Pimoroni RPi interfacer
sudo docker run \
    --name weather-pimoroni \
    -dv /rems/readings/weather.json:/rems/readings/weather.json \
    -v /rems/commands/transfer.sh:/rems/commands/transfer.sh \
    weather-pimoroni

# # Django webserver 
# sudo docker run \
#     --name weather-webserver \
#     -v /rems/readings/weather.json:/rems/readings/weather.json \
#     weather-webserver

# # Serial port parser
# sudo docker run \
#     --name residence-serial \
#     --device=/dev/ttyACM0:/dev/ttyACM0 \
#     -dv /rems/readings/residence.json:/rems/readings/residence.json \
#     residence-serial

# Obtain webserver IP as well as credentials such as username/password (can be edited in .env file)
read -p "Host (IP):" HOST
read -p "User:" USER
read -p "Password:" PASS
echo -e "HOST=$HOST\nUSER=$USER\nPASS:$PASS" > .env
