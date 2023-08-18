#!/bin/bash

source .env
ftp -inv $HOST << EOF
user $USER $PASS
put /rems/readings/weather.txt /rems/readings/weather.txt 
quit
EOF
