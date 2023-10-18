# Check for help flag
default_db_link="postgres://postgres:postgres@localhost:5433/pihpah"

bin="server"
service="pih-pah-${bin}"
dir="$(dirname "$(realpath "$0")")/"
remote_dir="/home/${USER}/pih-pah-deploy/${bin}/"

cd "${dir}../"
. ../../script/log.sh

log "${dir} running..."

if [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
  # echo "Can start only from project folder"
  printf "Usage: $0 [-h]"
  printf "Environment Variables:"
  printf "  USER\tSet the SSH destination as user\n"
  printf "  SERVER\tSet the SSH destination as server address\n"
  printf "  SERVER_PASSWORD\tPassword for user in remote host, I hope you do not use root\n"
  printf "  DATABASE_URL\tPostgresql link\n"
  printf "  PORT\tPort (default: 5000)\n"
  printf "  SSH_PRIVATE_KEY\tSsh private key\n"
  printf "  \tDefault: %s" "${default_db_link}"
  exit 0
fi

if [ -z "${USER}" ]; then
  error "USER must be set. Exiting."
  exit 1
fi

if [ -z "${SERVER}" ]; then
  error "SERVER must be set. Exiting."
  exit 1
fi

if [ -z "${SSH_PRIVATE_KEY}" ]; then
  error "SSH_PRIVATE_KEY must be set. Exiting."
  exit 1
fi

if [ -z "${SERVER_PASSWORD}" ]; then
  error "SERVER_PASSWORD must be set. Exiting."
  exit 1
fi

if [ -z "${DATABASE_URL}" ]; then
  DATABASE_URL="${default_db_link}"
fi

if [ -z "${PORT}" ]; then
  PORT=5000
fi

PASSWORD="${SERVER_PASSWORD}"

# Use an environment variable for the SSH user and server
SSH_DEST="${USER}@${SERVER}"

log 'building...'
cargo build --release
env CARGO_TARGET_DIR=../../target cargo build --release --bin server

tmp_ssh_private="$(mktemp)"
echo "${SSH_PRIVATE_KEY}" > "${tmp_ssh_private}"

# Transfer the Rust binary
log 'some ssh magic...'

ssh -i "${tmp_ssh_private}" "${SSH_DEST}" "mkdir -p ${remote_dir} && rm -f ${remote_dir}/${bin}" # if not exist
scp -i "${tmp_ssh_private}" "${dir}../../../target/release/${bin}" "${SSH_DEST}:${remote_dir}"

# Setup service
log 'connecting to server...'

temp_file="~/temp-${service}.service"

ssh -i "${tmp_ssh_private}" "${SSH_DEST}" <<EOF
  chmod +x  ${remote_dir}${bin}

  echo "[Unit]
Description=pih-pah ${bin}

[Service]
ExecStart=${remote_dir}/${bin} ${SERVER}:${PORT} 104.248.254.204:2007
Restart=always

[Install]
WantedBy=multi-user.target" > ${temp_file}

  printf '%s' "${PASSWORD}" | sudo -S -rm -f /etc/systemd/system/${service}.service
  printf '%s' "${PASSWORD}" | sudo -S mv ${temp_file} /etc/systemd/system/${service}.service
  printf '%s' "${PASSWORD}" | sudo -S systemctl daemon-reload 
  printf '%s' "${PASSWORD}" | sudo -S systemctl enable ${service} 
  printf '%s' "${PASSWORD}" | sudo -S systemctl start ${service} 
  printf '%s' "${PASSWORD}" | sudo -S systemctl restart ${service} 
EOF

rm -f "${temp_file}"
rm -f "${tmp_ssh_private}"