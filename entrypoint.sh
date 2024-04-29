#!/bin/sh
# This entrypoint is only use by Github actions 

report=$(reportly $1)
echo "REPORT<<EOF"$'\n'"$report"$'\n'EOF >> "$GITHUB_OUTPUT"