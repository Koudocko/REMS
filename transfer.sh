#!/bin/bash

HOST=192.168.3.132
USER=tyler
PASS=WEHRLE@$^*
LOCALFILE=/rems/readings/weather.txt
REMOTEPATH=/rems/readings/weather.txt

ftp -inv $HOST << EOF
user $USER $PASS
put $LOCALFILE $REMOTEPATH
quit
EOF
