FROM ubuntu:xenial
RUN sed -i '1ideb mirror://mirrors.ubuntu.com/mirrors.txt xenial main restricted universe multiverse' /etc/apt/sources.list
RUN sed -i '1ideb mirror://mirrors.ubuntu.com/mirrors.txt xenial-updates main restricted universe multiverse' /etc/apt/sources.list
RUN sed -i '1ideb mirror://mirrors.ubuntu.com/mirrors.txt xenial-backports main restricted universe multiverse' /etc/apt/sources.list
RUN sed -i '1ideb mirror://mirrors.ubuntu.com/mirrors.txt xenial-security main restricted universe multiverse' /etc/apt/sources.list
RUN apt-get update && apt-get install curl clang gcc postgresql postgresql-contrib sqlite3 libpq-dev libsqlite3-dev -q -y
RUN mkdir rust_install
WORKDIR /rust_install/
RUN curl -O https://static.rust-lang.org/dist/rust-1.13.0-x86_64-unknown-linux-gnu.tar.gz
RUN tar -xf rust-1.13.0-x86_64-unknown-linux-gnu.tar.gz
WORKDIR /rust_install/rust-1.13.0-x86_64-unknown-linux-gnu/
RUN chmod 755 install.sh
RUN ./install.sh
#RUN curl https://sh.rustup.rs -s >> rustup.sh
#RUN chmod 755 /rust_install/rustup.sh
#RUN ./rustup.sh -y --default-toolchain 1.13.0
ENV PATH=/root/.cargo/bin:$PATH
RUN cargo install diesel_cli
RUN mkdir /migration/
WORKDIR /migration/
RUN rm -r /rust_install/
COPY initdb.sql /migration/
WORKDIR /
RUN diesel setup --database-url='postgres://localhost/trip_info'