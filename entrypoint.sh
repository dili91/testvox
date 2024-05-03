#!/bin/bash
# This entrypoint is only use by Github actions

# Set default values for options
TEST_REPORTS_PATTERN=""
REPORT_TITLE=""

# Parse options and arguments
# inspired by https://medium.com/@Drew_Stokes/bash-argument-parsing-54f3b81a6a8f
while (( "$#" )); do
    case "$1" in
        --test-reports-pattern)
            if [ -n "$2" ] && [ ${2:0:1} != "-" ]; then
            TEST_REPORTS_PATTERN="$2"
            shift 2
            else
            echo "Error: Argument for $1 is missing" >&2
            exit 1
            fi
            ;;
        --report-title)
            if [ -n "$2" ] && [ ${2:0:1} != "-" ]; then
            REPORT_TITLE="$2"
            shift 2
            else
            echo "Error: Argument for $1 is missing" >&2
            exit 1
            fi
            ;;
        --include-skipped|--include-passed)
            if [ -n "$2" ] && [ ${2:0:1} != "-" ]; then
                if [ "$2" = true ] ; then
                    REPORTLY_OPTS+="$1 "
                fi
            shift 2
            else
            echo "Error: Argument for $1 is missing" >&2
            exit 1
            fi
            ;;
        -*|--*=) # unsupported flags
            echo "Error: Unsupported flag $1" >&2
            exit 1
            ;;
    esac
done

cat /github/workspace/README.md

ls -lah /github/workspace/app/build/test-results/**/*.xml

echo 'REPORT<<EOF' >> $GITHUB_OUTPUT
reportly $REPORTLY_OPTS --report-title "$REPORT_TITLE" --test-reports-pattern "$TEST_REPORTS_PATTERN"  >> $GITHUB_OUTPUT
echo 'EOF' >> $GITHUB_OUTPUT