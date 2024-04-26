FROM rust:1.77.2-slim-bookworm

WORKDIR /usr/src/junit-to-slack-reporter
RUN mkdir /junit-reports

COPY . .

RUN cargo install --path .

CMD ["junit-to-slack-reporter"]