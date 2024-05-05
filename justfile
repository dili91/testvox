alias db    := docker-build
alias dr    := docker-run
alias b     := build
alias r     := run
alias t     := test

default_test_reports_pattern := "./test-results/**/*.xml"

docker-build:
    docker build . -t testvox

docker-run test_reports_pattern=default_test_reports_pattern:
    docker run --rm --entrypoint /testvox/entrypoint.sh testvox \
    true false "My test repo" "/testvox/{{test_reports_pattern}}"

build:
    cargo build

run test_reports_pattern=default_test_reports_pattern:
    cargo run -- \
    --include-skipped \
    --report-title "A simple test report" \
    "{{test_reports_pattern}}"

test: 
    cargo nextest run