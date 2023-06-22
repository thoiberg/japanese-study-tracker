FROM rust:1.70

RUN mkdir /app
WORKDIR /app

# install binstall
RUN curl -L --proto '=https' --tlsv1.2 \
    -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
    | bash

# install watch
RUN rustup component add clippy
RUN cargo binstall -y cargo-watch
RUN cargo install diesel_cli

CMD [ "cargo", "watch", "-x", "'run'" ]