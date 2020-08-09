#!/bin/sh

. ./config.sh

printf "Testing telebar-server cleanup...\n"

if [ ! -f "$TELEBAR_SERVER" ]; then
    mkdir -p "$TEST_BUILD_PATH";
    cargo build --release --target-dir "$TEST_BUILD_PATH";
fi

$TELEBAR_SERVER --id 5 &
SERVER_PID=$!

wait_for "[ ! -e $SOCKET_PATH ]" 'FAILURE: Server did not create socket quickly enough.';

if ! kill $SERVER_PID; then
    printf "FAILURE: Could not kill server.\n";
    cleanup;
    exit 1;
fi

wait_for "[ -e $SOCKET_PATH ]" 'FAILURE: Server did not clean up quickly enough.';

printf "Server cleanup successful.\n";
