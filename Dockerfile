FROM rust:1.77.2-slim-bookworm

COPY . /reportly

RUN cargo install --path /reportly

ENTRYPOINT ["reportly"]