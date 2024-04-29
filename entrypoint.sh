#!/bin/sh
# This entrypoint is only use by Github actions 

report=$(reportly $1)
echo "report=$report" >> $GITHUB_OUTPUT