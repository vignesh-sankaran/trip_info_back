FROM liuchong/rustup:stable
RUN mkdir /app
WORKDIR /app
ADD . /app/
RUN cargo build
EXPOSE 20000
ENTRYPOINT ./target/debug/api
