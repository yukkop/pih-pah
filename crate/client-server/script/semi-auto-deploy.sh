# Check for help flag
default_ssh_port="22"
default_server_port="5000"

bin="server"
service="pih-pah-${bin}"
dir="$(dirname "$(realpath "$0")")/"

cd "${dir}../" || exit 1
. ../../script/log.sh

log "${dir} running..."

if [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
  printf "Usage: %s [-h]" "${dir}"
  printf "Environment Variables:"
  printf "  SSH_USER\tSet the SSH destination as user\n"
  printf "  SSH_ADDRESS\tSet the SSH destination as server address\n"
  printf "  SSH_USER_PASSWORD\tPassword for user in remote host, I hope you do not use root\n"
  printf "  SERVER_PORT\tPort\n"
  printf "  SSH_PRIVATE_KEY\tSsh private key\n"
  printf ""
  printf "  \tDefault:"
  printf "  \tSSH_SERVER_PORT %s" "${default_ssh_port}"
  printf "  \tSERVER_PORT %s" "${default_server_port}"
  exit 0
fi

if [ -z "${SSH_USER}" ]; then
  error "SSH_USER must be set. Exiting."
  exit 1
fi

if [ -z "${SSH_ADDRESS}" ]; then
  error "SSH_ADDRESS must be set. Exiting."
  exit 1
fi

if [ -z "${SSH_PRIVATE_KEY}" ]; then
  error "SSH_PRIVATE_KEY must be set. Exiting."
  exit 1
fi

if [ -z "${SSH_USER_PASSWORD}" ]; then
  error "SSH_USER_PASSWORD must be set. Exiting."
  exit 1
fi

if [ -z "${SERVER_PORT}" ]; then
  SERVER_PORT="${default_server_port}"
fi

if [ -z "${SSH_PORT}" ]; then
  SSH_PORT="${default_ssh_port}"
fi

# Use an environment variable for the SSH user and server
remote_dir="/home/${SSH_USER}/pih-pah-deploy/${bin}/"
SSH_DEST="${SSH_USER}@${SSH_ADDRESS}"

log 'building...'
if ! env CARGO_TARGET_DIR="${dir}../../target" cargo build --release --bin server; then
 error "build error"
 exit 1
fi

tmp_ssh_private="$(mktemp)"
echo "${SSH_PRIVATE_KEY}" > "${tmp_ssh_private}"

# Transfer the Rust binary
log 'some ssh magic...'

if ! ssh -o StrictHostKeyChecking=no -p "${SSH_PORT}" -i "${tmp_ssh_private}" "${SSH_DEST}" "mkdir -p ${remote_dir} && rm -f ${remote_dir}/${bin}"; then
 error "ssh error"
 exit 1
fi

if ! scp -o StrictHostKeyChecking=no -P "${SSH_PORT}" -i "${tmp_ssh_private}" "${dir}../../../target/release/${bin}" "${SSH_DEST}:${remote_dir}"; then
 error "ssh error"
 exit 1
fi

# Setup service
log 'connecting to server...'

TEMP_SERVICE="$(mktemp)"
PASSWORD="${SSH_USER_PASSWORD}"

# shellcheck disable=SC2087
ssh -o StrictHostKeyChecking=no -p "${SSH_PORT}" -i "${tmp_ssh_private}" "${SSH_DEST}" <<EOF
  chmod +x  ${remote_dir}${bin}

  echo "[Unit]
Description=pih-pah ${bin}

[Service]
ExecStart=${remote_dir}/${bin} ${SSH_ADDRESS}:${SERVER_PORT} 104.248.254.204:2007
Restart=always

[Install]
WantedBy=multi-user.target" > ${TEMP_SERVICE}

  printf '%s' "${PASSWORD}" | sudo -S -rm -f /etc/systemd/system/${service}.service
  printf '%s' "${PASSWORD}" | sudo -S mv ${TEMP_SERVICE} /etc/systemd/system/${service}.service
  printf '%s' "${PASSWORD}" | sudo -S systemctl daemon-reload 
  printf '%s' "${PASSWORD}" | sudo -S systemctl enable ${service} 
  printf '%s' "${PASSWORD}" | sudo -S systemctl start ${service} 
  printf '%s' "${PASSWORD}" | sudo -S systemctl restart ${service}

  rm -f "${TEMP_SERVICE}"
EOF
# shellcheck disable=SC2181
if [ $? -ne 0 ]; then
 error "ssh error"
 exit 1
fi

rm -f "${tmp_ssh_private}"