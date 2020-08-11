#!/bin/sh
. ./config.sh

printf "%s\n%s" "battery" "Battery update!" | nc -U "$XDG_RUNTIME_DIR"/"$TELEBAR_SERVER_ID"_telebar_socket -w0
