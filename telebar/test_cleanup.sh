#!/bin/sh
# Killing processes from within `Cargo test` is somewhat flakey, so we do it
# from within a shell-script instead

TELEBAR_SERVER="$PWD"/target/release/telebar
SOCKET_PATH=$XDG_RUNTIME_DIR/5_telebar_socket;

cleanup() {
    rm -rf "$SOCKET_PATH";
}

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

cleanup;

printf "Testing telebar-server cleanup...\n"

if [ ! -f "$TELEBAR_SERVER" ]; then
  printf "FAILURE: Could not find server binary.\n";
  printf "Please create a release build before running this test.\n"
  cleanup;
  exit 1
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
