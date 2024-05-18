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

check-crate-version-available version:
    #!/usr/bin/env sh
    response=$(curl -s -o /dev/null -w "%{http_code}" https://crates.io/api/v1/crates/testvox/{{version}})
    if [[ $response == 2* ]]; then
        echo "Version {{version}} is already published on crates.io"
        exit 1
    fi