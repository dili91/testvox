FROM rust:1.77.2-slim-bookworm

COPY . /testvox

RUN cargo install --path /testvox

ENTRYPOINT ["testvox"]