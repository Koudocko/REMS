# NOT DONE, finding dependencies
#
#
#
##!/bin/sh

#// Install dependencies
#sudo apt-get install cron
#sudo apt-get install python3
#wget https://github.com/pimoroni/breakout-garden/archive/master.zip
#unzip master.zip
#sudo ./breakout-garden-master/install.sh
#sudo systemctl enable cron --now

#// Install files
#sudo mkdir -p /rems/readings
#sudo mkdir -p /rems/commands
#sudo touch /rems/readings/log0.txt
#sudo touch /rems/readings/log1.txt
#sudo chmod 777 /rems/readings
#sudo cp rems_2_readings.py /rems/commands/rems_2_readings.py
#sudo cp rems_2_readings.sh /rems/commands/rems_2_readings.sh
#sudo crontab -l > cron_temp
#echo "* * * * * python3 /rems/commands/rems_2_transfer.py"
#sudo crontab cron_temp
#sudo rm cron_temp
