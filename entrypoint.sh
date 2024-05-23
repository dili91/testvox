#!/bin/bash
# This entrypoint is only use by Github actions
# and leverages an ad-hoc positional notation for arguments

if [ "$#" -lt 4 ] || [ "$#" -gt 5 ]; then
    printf "Invalid positional arguments.\nExpected: \$include_skipped \$include_passed \$title \$reports_patterns [\$link]" >&2
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
testvox $OPTS --title "$3" --reports-pattern "$4" --link "$5">> $GITHUB_OUTPUT
echo 'EOF' >> $GITHUB_OUTPUT

