alias db    := docker-build
alias dr    := docker-run
alias b     := build
alias r     := run
alias f     := format
alias t     := test

default_test_reports_patterns := "./test-results/**/*.xml"

docker-build:
    docker build . -t testvox

docker-run test_reports_patterns=default_test_reports_patterns:
    docker run --rm --entrypoint /testvox/entrypoint.sh testvox \
    true false "My test repo" "/testvox/{{test_reports_patterns}}"

build:
    cargo build

format:
    cargo fmt

test: 
    cargo test

run test_reports_patterns=default_test_reports_patterns:
    cargo run -- \
    --include-skipped \
    --title "A simple test report" \
    "{{test_reports_patterns}}"
