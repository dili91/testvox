FROM rust:1.77.2-slim-bookworm

COPY . /usr/src/reportly

RUN cargo install --path /usr/src/reportly

CMD ["reportly"]