FROM rust:1.85-slim-bookworm

COPY . /testvox

RUN cargo install --path /testvox

ENTRYPOINT ["testvox"]