#!/bin/sh
# This entrypoint is only use by Github actions

# Set default values for options
REPORTLY_OPTS=""
REPORTLY_ARGS=""

# Parse options and arguments
# inspired by https://medium.com/@Drew_Stokes/bash-argument-parsing-54f3b81a6a8f
while (( "$#" )); do
    case "$1" in
        --test-reports-pattern|--report-title)
            if [ -n "$2" ] && [ ${2:0:1} != "-" ]; then
            REPORTLY_ARGS+=" $1 $2"
            shift 2
            else
            echo "Error: Argument for $1 is missing" >&2
            exit 1
            fi
            ;;
        --include-skipped|--include-passed)
            if [ -n "$2" ] && [ ${2:0:1} != "-" ]; then
                if [ "$2" = true ] ; then
                    REPORTLY_OPTS+=" $1"
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

echo 'REPORT<<EOF' >> $GITHUB_OUTPUT
reportly "$REPORTLY_OPTS" "$REPORTLY_ARGS" >> $GITHUB_OUTPUT
echo 'EOF' >> $GITHUB_OUTPUT