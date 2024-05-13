<p align="center">
  <img src='./images/sample.png'/>
</p>

# Testvox: turns test reports into simple, human readable summaries

Testvox is tiny Rust library with a very simple objective: turning test reports into human friendly messages.
The project simply deals with reports generation, it does not care about *sending* those reports.

At the minute, it only helps turning test results in Junit format into Slack messages, but I'd like add more parsers and reporters in future.

# Use within CI

Currently, only Github Actions are supported.

## Use as Github action

#TODO


# Todo

## `Next releases
  - [ ] Add link to tests failing on CI
  - [ ] Support other test frameworks: js, rust,net...
    - [ ] Test reports detection
  - [ ] Support usage as CircleCI orb
  - [ ] Supports defining multiple reports patterns via GH action

