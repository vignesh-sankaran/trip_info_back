#!/bin/bash
DB=admin
/usr/bin/mongod --dbpath /data --nojournal &
while ! netcat -vz localhost 27017; do sleep 1; done
mongo --eval "db.createCollection('users', { id: 'string' })"
mongo $DB --eval "db.createUser({ user: 'system', pwd: 'system', roles: [ { role: 'dbAdminAnyDatabase', db: '$DB' } ] });"
