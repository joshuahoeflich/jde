#!/bin/sh

cleanup() {
    rm -rf "$TELEBAR_TEST_SOCKET_PATH";
}

# Tests a condition for ~0.5 seconds before failing.
# $1 == condition to wait for
# $2 == failure message
wait_for(){
    COUNTER=0;
    while eval "$1"; do
        sleep 0.1;
        if [ $COUNTER -gt 3 ]; then
            printf "%s\n" "$2";
            cleanup;
            exit 1
        else
            COUNTER=$((COUNTER+1));
        fi
    done
    COUNTER=0;
}
