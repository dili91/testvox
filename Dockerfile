FROM rust:1.77.2-slim-bookworm

WORKDIR /home/reportly
RUN mkdir test-reports

COPY . .

RUN cargo install --path .

CMD ["reportly"]