# Check for help flag
default_db_link="postgres://postgres:postgres@localhost:5433/pih-pah"

if [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
  # echo "Can start only from project folder"
  printf "Usage: $0 [-h]"
  printf "Environment Variables:"
  printf "  USER_AT_SERVER_ENV\tSet the SSH destination as user@server\n"
  printf "  SERVER_PASSWORD\tpassword for user in remote host, I hope you do not use root\n"
  printf "  DATABASE_URL\tpostgresql link\n"
  printf "  \tdefault: %s" "${default_db_link}"
  exit 0
fi

# Ensure the env variable is set
if [ -z "${USER_AT_SERVER_ENV}" ]; then
  echo "USER_AT_SERVER_ENV must be set. Exiting."
  exit 1
fi

if [ -z "${SERVER_PASSWORD}" ]; then
  echo "SERVER_PASSWORD must be set. Exiting."
  exit 1
fi

if [ -z "${DATABASE_URL}" ]; then
  DATABASE_URL="${default_db_link}"
fi

PASSWORD="${SERVER_PASSWORD}"

# Use an environment variable for the SSH user and server
SSH_DEST="${USER_AT_SERVER_ENV}"

dir="$(dirname "$(realpath "$0")")/"
remote_dir="/home/yukkop/pih-pah-deploy/receiver/"
bin="receiver"
service="pih-pah-receiver"

cd "${dir}../"
cargo build --release

# Transfer the Rust binary
ssh "${SSH_DEST}" "mkdir -p ${remote_dir} && rm -f ${remote_dir}/${bin}" # if not exist
scp "${dir}../../../target/release/${bin}" "${SSH_DEST}:${remote_dir}"

# ssh "${SSH_DEST}" <<EOF printf '%s' "${PASSWORD}" | sudo -S ls /
# EOF

temp_file="~/temp-${service}.service"

# SSH and setup service
ssh "${SSH_DEST}" <<EOF
  chmod +x  ${remote_dir}${bin}

  echo "[Unit]
Description=pih-pah reciever

[Service]
ExecStart=env DATABASE_URL=${DATABASE_URL} ${remote_dir}/${bin} 2007
Restart=always

[Install]
WantedBy=multi-user.target" > ${temp_file}

  printf '%s' "${PASSWORD}" | sudo -S -rm -f /etc/systemd/system/${service}.service
  printf '%s' "${PASSWORD}" | sudo -S mv ${temp_file} /etc/systemd/system/${service}.service
  printf '%s' "${PASSWORD}" | sudo -S systemctl daemon-reload 
  printf '%s' "${PASSWORD}" | sudo -S systemctl enable ${service} 
  printf '%s' "${PASSWORD}" | sudo -S systemctl start ${service} 
  # rm -f ${temp_file}
EOF
