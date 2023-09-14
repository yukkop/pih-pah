#!/bin/sh

set -u

: "${CARGO_TARGET_DIR:=target}"

server="pih-pah@${SERVER_IP}"

if ! [ "$(md5sum "${CARGO_TARGET_DIR}/debug/server" | awk '{ print $1 }')" = "$(ssh "${server}" md5sum '/srv/pih-pah/pih-pah-server' | awk '{ print $1 }')" ]; then
  if ! scp "${CARGO_TARGET_DIR}/debug/server" "${server}:/srv/pih-pah/pih-pah-server"; then
    printf >&2 "deploy-server.sh: ERROR: failed to copy binary to the server\n"
    exit 1
  fi
  if ! scp -r "assets" "${server}:/srv/pih-pah/assets"; then
    printf >&2 "deploy-server.sh: ERROR: failed to copy assets to the server\n"
    exit 1
  fi

  wait
fi

ssh "${server}" sudo systemctl restart 'pih-pah-server.service'
