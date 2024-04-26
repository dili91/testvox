FROM rust:1.77.2-slim-bookworm

WORKDIR /usr/src/reportly
RUN mkdir /junit-reports

COPY . .

RUN cargo install --path .

CMD ["reportly"]