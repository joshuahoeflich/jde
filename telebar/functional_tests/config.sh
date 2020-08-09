#!/bin/sh

export TEST_BUILD_PATH=$PWD/test_build
export TELEBAR_SERVER=$TEST_BUILD_PATH/release/main
export SOCKET_PATH=$XDG_RUNTIME_DIR/5_telebar_socket;

cleanup() {
    rm -rf "$SOCKET_PATH";
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
