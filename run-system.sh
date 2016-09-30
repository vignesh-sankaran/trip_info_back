#/bin/bash
cd api && \
./prerun-config.sh && \
cd .. && \
docker-compose build && \
docker-compose up
