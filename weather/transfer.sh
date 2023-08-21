#!/bin/bash

source .env
ftp -inv $HOST << EOF
user $USER $PASS
put /rems/readings/weather.json /rems/readings/weather.json 
quit
EOF
