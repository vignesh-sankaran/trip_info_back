docker rm -f rustapi
docker build -t rust:api ./ && \
docker run -d -p 20000:20000 -h localhost --name rustapi rust:api