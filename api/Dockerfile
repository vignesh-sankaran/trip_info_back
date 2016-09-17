FROM scorpil/rust:1.11
RUN mkdir /app
WORKDIR /app
ADD . /app/
RUN cargo build
EXPOSE 20000
ENTRYPOINT ./target/debug/api
