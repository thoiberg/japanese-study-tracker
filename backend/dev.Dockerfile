FROM rust:1.80

RUN mkdir /app
WORKDIR /app

# install binstall
RUN curl -L --proto '=https' --tlsv1.2 \
    -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
    | bash

RUN apt-get update
RUN apt-get install -y redis-server

# install cargo bins
RUN rustup component add clippy rustfmt
RUN cargo binstall -y cargo-watch cargo-nextest

CMD [ "cargo", "watch", "-x", "'run'" ]