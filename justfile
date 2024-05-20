alias db    := docker-build
alias dr    := docker-run
alias b     := build
alias r     := run
alias f     := format
alias t     := test

project_version:= `cargo metadata --format-version=1 --no-deps | jq -r '.packages[0].version'`
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

get-project-version:
    @echo {{project_version}}

check-crate-version-available:
    #!/bin/bash
    response=$(curl -s -o /dev/null -w "%{http_code}" https://crates.io/api/v1/crates/testvox/{{project_version}})
    if [[ $response == 2* ]]; then
        echo "Version {{project_version}} is already published on crates.io"
        exit 1
    fi