FROM scorpil/rust:1.11
RUN mkdir /app
WORKDIR /app
ADD . /app/
RUN cargo build
ENTRYPOINT cargo run
