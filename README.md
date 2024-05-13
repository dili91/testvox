<p align="center">
  <img src='./images/sample.png'/>
</p>

# Testvox: turns test reports into simple, human readable summaries

Testvox is tiny Rust library with a very simple objective: turning test reports into messages that and human friendly and ready to be shared.
The project simply deals with summaries generation, it does not care about sending those reports.

At the minute, it only helps turning test results in Junit format into Slack messages, but I'd like add more parsers and reporters in future.

# Use within CI

Currently, only Github Actions are supported.

## Use as Github action

#TODO


# Todo
## `v0.1.0`
  - [x] Remove code that panics
  - [x] improve test coverage
  - [ ] Review methods visibility
  - [x] Deterministic ordering: failed, skipped, passed
  - [ ] Improve CI build, linting, ecc
  - [ ] verify extendibility 
  - [x] Support setting passed and/or failed via flag
  - [x] Test with real project
  - [ ] Review License, contributing (PRs and Issues)

## `Next releases
  - [ ] Add link to test failing on CI
  - [ ] Support other test frameworks: js, .net...
    - [ ] Test reports detection
  - [ ] Supports defining multiple reports patterns via GH action

