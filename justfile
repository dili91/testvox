alias db    := docker-build
alias dr    := docker-run
alias b     := build
alias r     := run
alias t     := test

default_test_reports_pattern := "./test-reports/**/*.xml"

docker-build:
    docker build . -t reportly

docker-run:
    docker run --rm -v "./test-reports:/home/reportly/test-reports" reportly reportly "{{default_test_reports_pattern}}"

build:
    cargo build

run test_reports_pattern=default_test_reports_pattern:
    cargo run -- "{{test_reports_pattern}}"

test: 
    cargo nextest run