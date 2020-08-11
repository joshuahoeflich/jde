#!/bin/sh
. "$TELEBAR_TEST_SHELL_LIB"

printf "Testing telebar-server cleanup...\n"

cargo build --release --target-dir "$TELEBAR_TEST_BUILD_DIR";

$TELEBAR_TEST_BINARY --id "$TELEBAR_TEST_SOCKET_ID" --output newlines --config "$TELEBAR_TEST_CONFIG_FILE" &
SERVER_PID=$!

wait_for "[ ! -e $TELEBAR_TEST_SOCKET_PATH ]" 'FAILURE: Server did not create socket quickly enough.';

if ! kill $SERVER_PID; then
    printf "FAILURE: Could not kill server.\n";
    cleanup;
    exit 1;
fi

wait_for "[ -e $TELEBAR_TEST_SOCKET_PATH ]" 'FAILURE: Server did not clean up quickly enough.';

printf "Server cleanup successful.\n";
