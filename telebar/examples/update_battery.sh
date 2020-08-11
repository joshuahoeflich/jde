#!/bin/sh
printf "%s\n%s" "battery" "This actually works!" | socat -t0 ABSTRACT-CONNECT:"$TELEBAR_EXAMPLE_SOCKET_ID"_telebar_socket STDIN
