#!/bin/sh

// Install dependencies
sudo apt-get install cron
sudo apt-get install python3
wget https://github.com/pimoroni/breakout-garden/archive/master.zip
unzip master.zip
sudo ./breakout-garden-master/install.sh
sudo rm -r breakout-garden-master
sudo systemctl enable cron --now

// Install files
sudo mkdir -p /rems/readings
sudo mkdir -p /rems/commands
sudo touch /rems/readings/log0.txt
sudo cp rems_2_readings.py /rems/commands/rems_2_readings.py
sudo cp rems_2_readings.sh /rems/commands/rems_2_readings.sh
sudo crontab -l > cron_temp
echo "* * * * * python3 /rems/commands/rems_2_transfer.py"
sudo crontab cron_temp
sudo rm cron_temp
