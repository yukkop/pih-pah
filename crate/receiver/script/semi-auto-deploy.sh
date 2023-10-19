# Check for help flag
default_db_link='postgres://postgres:postgres@localhost:5433/pih-pah'
default_port='22'

bin='receiver'
service="pih-pah-${bin}"
dir="$(dirname "$(realpath "$0")")/"
name="$(basename "$0")"

cd "${dir}../../../" || exit 1 # going to repos root
. ./script/log.sh

if [ "$1" = '-h' ] || [ "$1" = '--help' ]; then
  printf '\033[33mWARNING\033[0m: it uses sudo in remote server..\n'
  printf '\033[33mWARNING\033[0m: if you want start it on another os need improves..\n'
  printf '\n'
  printf 'Usage: %s [-h]\n' "${name}"
  printf 'script for deploying %s target\n' "${bin}"
  printf '\n'
  printf 'Options:\n'
  printf '  --help / -h    this message\n'
  printf 'Environment Variables:\n'
  printf '  SSH_USER  Set the SSH destination as user\n'
  printf '  SSH_ADDRESS  Set the SSH destination as server address\n'
  printf '  SSH_PORT  Set the SSH destination as server port\n'
  printf '  SSH_USER_PASSWORD  password for user in remote host, I hope you do not use root\n'
  printf '  SSH_PRIVATE_KEY  Ssh private key\n'
  printf '  DATABASE_URL  postgresql link\n'
  printf '\n'
  printf '  defaults:\n'
  printf '    DATABASE_URL: %s\n' "${default_db_link}"
  printf '    SSH_PORT: %s\n' "${default_port}"
  exit 0
fi

log "${name} running..."
log 'check env...'

if [ -z "${SSH_USER}" ]; then
  error 'SSH_USER must be set. Exiting.'
  exit 1
fi

if [ -z "${SSH_ADDRESS}" ]; then
  error 'SSH_ADDRESS must be set. Exiting.'
  exit 1
fi

if [ -z "${SSH_PRIVATE_KEY}" ]; then
  error 'SSH_PRIVATE_KEY must be set. Exiting.'
  exit 1
fi

if [ -z "${SSH_USER_PASSWORD}" ]; then
  error 'SSH_USER_PASSWORD must be set. Exiting.'
  exit 1
fi

if [ -z "${DATABASE_URL}" ]; then
  DATABASE_URL="${default_db_link}"
fi

if [ -z "${SSH_PORT}" ]; then
  SSH_PORT="${default_port}"
fi

# Use an environment variable for the SSH user and server
remote_dir="/home/${SSH_USER}/pih-pah-deploy/${bin}/"
SSH_DEST="${SSH_USER}@${SSH_ADDRESS}"

log 'building...'
if ! env CARGO_TARGET_DIR="${dir}/target" cargo build --release --bin "${bin}"; then
 error 'build error'
 exit 1
fi

# Ssh setup
tmp_ssh_private="$(mktemp)"
echo "${SSH_PRIVATE_KEY}" > "${tmp_ssh_private}"

# Transfer the Rust binary
log 'some ssh magic...'
if ! ssh -o StrictHostKeyChecking=no -p "${SSH_PORT}" -i "${tmp_ssh_private}" "${SSH_DEST}" "mkdir -p ${remote_dir} && rm -f ${remote_dir}/${bin}"; then
 error 'ssh error'
 exit 1
fi

if ! scp -o StrictHostKeyChecking=no -P "${SSH_PORT}" -i "${tmp_ssh_private}" "${dir}/target/release/${bin}" "${SSH_DEST}:${remote_dir}"; then
 error 'ssh error'
 exit 1
fi


# SSH and setup service
log 'connecting to server...'

TEMP_SERVICE="$(mktemp)"
PASSWORD="${SSH_USER_PASSWORD}"

# shellcheck disable=SC2087
ssh -o StrictHostKeyChecking=no -p "${SSH_PORT}" -i "${tmp_ssh_private}" "${SSH_DEST}" <<EOF
  printf '%s' "${PASSWORD}" | sudo pacman -S alsa-lib
  chmod +x  ${remote_dir}${bin}

  echo "[Unit]
Description=pih-pah ${bin}

[Service]
ExecStart=env DATABASE_URL=${DATABASE_URL} ${remote_dir}/${bin} 2007
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
 error 'ssh error'
 exit 1
fi

rm -f "${tmp_ssh_private}"