#!/bin/sh
TELEBAR_SERVER_ID=5

printf "%s\n%s" "battery" "Third test" | nc -U "$XDG_RUNTIME_DIR"/"$TELEBAR_SERVER_ID"_telebar_socket -w0
