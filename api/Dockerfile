FROM ubuntu:xenial
RUN apt-get update && apt-get install curl build-essential ca-certificates file xutils-dev openssl libssl-dev -y
RUN mkdir /rust
WORKDIR /rust
RUN curl https://sh.rustup.rs -s >> rustup.sh
RUN chmod 755 /rust/rustup.sh
RUN ./rustup.sh -y --default-toolchain 1.14.0
ENV PATH=/root/.cargo/bin:$PATH SSL_VERSION=1.0.2g
RUN mkdir /app
WORKDIR /app
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock
COPY prebuild.rs /app/prebuild.rs
COPY build.rs /app/build.rs
COPY /src/serde_types.in.rs /app/src/serde_types.in.rs
RUN cargo build --lib
COPY /src/main.rs /app/src/main.rs
COPY /ssl /app/ssl
RUN cargo build --release
EXPOSE 20000
CMD ./target/release/api