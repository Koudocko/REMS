#!/bin/bash

HOST=192.168.3.132
USER=tyler
PASS=WEHRLE@$^*
LOCALFILE=/rems/readings/log0.txt
REMOTEPATH=/rems/readings/log0.txt

ftp -inv $HOST << EOF
user $USER $PASS
put $LOCALFILE $REMOTEPATH
quit
EOF
