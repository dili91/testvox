alias db    := docker-build
alias dr    := docker-run
alias b     := build
alias r     := run
alias t     := test

default_test_reports_pattern := "./test-reports/**/*.xml"

docker-build:
    docker build . -t reportly

docker-run:
    docker run --rm --entrypoint /reportly/entrypoint.sh reportly --include-skipped true --report-title "a title" --test-reports-pattern "/reportly/{{default_test_reports_pattern}}"

build:
    cargo build

run test_reports_pattern=default_test_reports_pattern:
    cargo run -- \
    --include-skipped \
    --report-title "A simple test report" \
    --test-reports-pattern "{{test_reports_pattern}}"

test: 
    cargo nextest run