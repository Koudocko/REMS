#!/bin/sh

# APT packages
sudo apt-get update
sudo apt-get install python3 python3-pip apache2 libapache2-mod-wsgi-py3

# Django
pip install django
django-admin

# Breakout garden
wget https://github.com/pimoroni/breakout-garden/archive/master.zip
unzip master.zip
sudo ./breakout-garden-master/install.sh
sudo rm -r breakout-garden-master

# Cron job
sudo apt-get install cron
sudo systemctl enable cron --now
sudo crontab -l > cron_temp
echo "* * * * * python3 /rems/commands/weather.py"
sudo crontab cron_temp
sudo rm cron_temp

# Install Files
sudo mkdir -p /rems/readings
sudo mkdir -p /rems/commands
sudo touch /rems/readings/weather.txt
sudo chmod -R 777 /rems/readings
sudo cp weather.py /rems/commands/weather.py
sudo cp transfer.sh /rems/commands/transfer.sh

# Transfer
read -p "Host (IP):" HOST
read -p "User:" USER
read -p "Password:" PASS
echo -e "HOST=$HOST\nUSER=$USER\nPASS:$PASS" > .env
