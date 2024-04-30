#!/bin/sh
# This entrypoint is only use by Github actions 

echo 'REPORT<<EOF' >> $GITHUB_OUTPUT
reportly --report-title "$1" --test-reports-pattern "$2" >> $GITHUB_OUTPUT
echo 'EOF' >> $GITHUB_OUTPUT