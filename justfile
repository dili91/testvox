alias db    := docker-build
alias dr    := docker-run
alias b     := build
alias r     := run
alias t     := test

docker-build:
    docker build . -t reportly

docker-run:
    @docker run --rm -v "./test-reports/basic:/home/reportly/test-reports/" reportly

build:
    cargo build

run:
    cargo run

test: 
    cargo nextest run