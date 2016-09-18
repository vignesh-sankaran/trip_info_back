FROM ubuntu:xenial
RUN apt-get update && apt-get install curl build-essential ca-certificates file xutils-dev nmap -y
RUN mkdir /rust
WORKDIR /rust
RUN curl https://sh.rustup.rs -s >> rustup.sh
RUN chmod 755 /rust/rustup.sh
RUN ./rustup.sh -y
ENV PATH=/root/.cargo/bin:$PATH SSL_VERSION=1.0.2h
RUN rustup default 1.11.0
RUN curl https://www.openssl.org/source/openssl-$SSL_VERSION.tar.gz -O && \
    tar -xzf openssl-$SSL_VERSION.tar.gz && \
    cd openssl-$SSL_VERSION && ./config && make depend && make install && \
    cd .. && rm -rf openssl-$SSL_VERSION*
ENV OPENSSL_LIB_DIR=/usr/local/ssl/lib \
    OPENSSL_INCLUDE_DIR=/usr/local/ssl/include \
    OPENSSL_STATIC=1
RUN mkdir /app
WORKDIR /app
ADD . /app/
RUN cargo build
EXPOSE 20000
CMD ./target/debug/api
