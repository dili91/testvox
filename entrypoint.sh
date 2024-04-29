#!/bin/sh
# This entrypoint is only use by Github actions 

echo 'REPORT<<EOF' >> $GITHUB_OUTPUT
reportly $1 >> $GITHUB_OUTPUT
echo 'EOF' >> $GITHUB_OUTPUT