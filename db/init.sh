#!/bin/bash
DB=admin
/usr/bin/mongod --dbpath /data --nojournal &
while ! netcat -vz localhost 27017; do sleep 1; done
mongo ./initdb.js
