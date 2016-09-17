docker rm -f rustapi && \
docker build -t rust:api ./ && \
docker run -d -p 20000:20000 -P --name rustapi rust:api
