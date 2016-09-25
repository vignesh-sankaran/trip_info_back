FROM ubuntu:xenial
RUN apt-get update && apt-get install curl build-essential ca-certificates file xutils-dev nmap -y
RUN mkdir /rust
WORKDIR /rust
RUN curl https://sh.rustup.rs -s >> rustup.sh
RUN chmod 755 /rust/rustup.sh
RUN ./rustup.sh -y --default-toolchain 1.11.0
ENV PATH=/root/.cargo/bin:$PATH SSL_VERSION=1.0.2h
RUN curl https://www.openssl.org/source/openssl-$SSL_VERSION.tar.gz -O && \
    tar -xzf openssl-$SSL_VERSION.tar.gz && \
    cd openssl-$SSL_VERSION && ./config && make depend && make install && \
    cd .. && rm -rf openssl-$SSL_VERSION*
ENV OPENSSL_LIB_DIR=/usr/local/ssl/lib \
    OPENSSL_INCLUDE_DIR=/usr/local/ssl/include \
    OPENSSL_STATIC=1
RUN mkdir /app
WORKDIR /app
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock
COPY prebuild.rs /app/prebuild.rs
RUN cargo build --lib
COPY /src /app/src
RUN cargo build
EXPOSE 20000
CMD ./target/debug/api
