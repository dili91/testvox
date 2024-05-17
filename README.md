<p align="center">
  <img src='./images/sample.png'/>
</p>

# Testvox: turns test reports into simple, human readable summaries

Testvox is tiny Rust library with a very simple objective: turning test reports into human readable summaries, to be shared on common messaging apps.
The project simply deals with reports generation, it does not care about *sending* those reports.

At the minute, it only helps turning test results in Junit format into Slack messages, but I'd like add more parsers and reporters in future.

Its primary use case is probably within CI pipelines, although it can be used as CLI and as library as well. 

# Use within CI

Currently, only Github Actions are supported.

## Use as Github action

To use this as Github action it is enough to place the following step after your tests are generated and right before sending the message:
```yaml
  steps:

  # ... Steps that generate test results ...

  - uses: dili91/testvox@v0.1.0
    name: Generate Slack report from Junit results
    # if: always() // might be needed, depending on your pipeline
    id: generate_slack_report
    with:
      include_skipped: true
      reports_pattern: "./test-results/**/*.xml"

  # ... Step that sends the report ...
```

Below, and in the [acceptance-test.yml](./.github/workflows/acceptance_tests.yml) file you can find a full example: 

```yaml

on: [push]

jobs:
  tests:
    name: Acceptance Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dili91/testvox@v0.1.0
        name: Generate Slack report from Junit results
        # if: always() // might be needed, depending on your pipeline
        id: generate_slack_report
        with:
          include_skipped: true
          reports_pattern: "./test-results/**/*.xml"
      - name: Send Slack report
        uses: slackapi/slack-github-action@v1.26.0
        with:
          payload: ${{steps.generate_slack_report.outputs.report}}
        env:
          SLACK_WEBHOOK_URL: ${{ secrets.SLACK_WEBHOOK_URL }}
          SLACK_WEBHOOK_TYPE: INCOMING_WEBHOOK

```

### Default configuration

The Github action has the following requirements and defaults values: 

| Name            | Required           | Default                                |
|-----------------|--------------------|----------------------------------------|
| title           | :white_check_mark: | `${{ github.repository }} test report` |
| reports_pattern | :white_check_mark: | `./build/test-results/*.xml`           |
| include_skipped | :x:                | false                                  |
| include_passed  | :x:                | false                                  |

# Use as CLI

Install the library locally:

```shell
cargo install testvox
```

then, invoke it from your terminal:

```shell
testvox --help

Usage: testvox [OPTIONS] <TITLE> [REPORTS_PATTERN]...

Arguments:
  <TITLE>               The title of the test report
  [REPORTS_PATTERN]...  The test report pattern to look for [default: ./build/test-results/**/*.xml,./app/build/test-results/**/*.xml]

Options:
      --include-skipped  Whether to include skipped tests in the report
      --include-passed   Whether to include passed tests in the report
  -h, --help             Print help
```

# Use as library

Testvox can be also used as library when needed. You can install it by adding the crate to your project: 

```shell
cargo add testvox
```

You can then refer to its [create_test_report](https://docs.rs/testvox/latest/testvox/fn.create_test_report.html) function to start using it.