#!/bin/bash
# This entrypoint is only use by Github actions
# and leverages an ad-hoc positional notation for arguments

if [ "$#" -ne 4 ]; then
    printf "Invalid positional arguments.\nExpected: \$include_skipped \$include_passed \$report_title \$report_patterns" >&2
    exit 1
fi

OPTS=""
if [ "$1" = true ] ; then
    OPTS+="--include-skipped"
fi
if [ "$2" = true ] ; then
    OPTS+=" --include-passed"
fi

echo 'REPORT<<EOF' >> $GITHUB_OUTPUT
reportly $OPTS --report-title "$3" "$4" >> $GITHUB_OUTPUT
echo 'EOF' >> $GITHUB_OUTPUT

