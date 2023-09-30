# Check for help flag
default_db_link="postgres://postgres:postgres@localhost:5433/pihpah"

if [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
  # echo "Can start only from project folder"
  printf "Usage: $0 [-h]"
  printf "Environment Variables:"
  printf "  USER\tSet the SSH destination as user\n"
  printf "  SERVER\tSet the SSH destination as server addres\n"
  printf "  SERVER_PASSWORD\tpassword for user in remote host, I hope you do not use root\n"
  printf "  DATABASE_URL\tpostgresql link\n"
  printf "  \tdefault: %s" "${default_db_link}"
  exit 0
fi

if [ -z "${USER}" ]; then
  echo "USER must be set. Exiting."
  exit 1
fi

if [ -z "${SERVER}" ]; then
  echo "SERVER must be set. Exiting."
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
SSH_DEST="${USER}@${SERVER}"

dir="$(dirname "$(realpath "$0")")/"
remote_dir="/home/${USER}/pih-pah-deploy/load-balancer/"
bin="load-balancer"
service="pih-pah-load-balancer"

cd "${dir}../"
cargo build --release

# Transfer the Rust binary
ssh "${SSH_DEST}" "mkdir -p ${remote_dir} && rm -f ${remote_dir}/${bin}" # if not exist
scp "${dir}../../../target/release/${bin}" "${SSH_DEST}:${remote_dir}"

temp_file="~/temp-${service}.service"

# SSH and setup service
ssh "${SSH_DEST}" <<EOF
  chmod +x  ${remote_dir}${bin}

  echo "[Unit]
Description=pih-pah reciever

[Service]
ExecStart=env DATABASE_URL=${DATABASE_URL} ROCKET_ADDRESS=0.0.0.0 ${remote_dir}/${bin}
Restart=always

[Install]
WantedBy=multi-user.target" > ${temp_file}

  printf '%s' "${PASSWORD}" | sudo -S -rm -f /etc/systemd/system/${service}.service
  printf '%s' "${PASSWORD}" | sudo -S mv ${temp_file} /etc/systemd/system/${service}.service
  printf '%s' "${PASSWORD}" | sudo -S systemctl daemon-reload 
  printf '%s' "${PASSWORD}" | sudo -S systemctl enable ${service} 
  printf '%s' "${PASSWORD}" | sudo -S systemctl start ${service} 
  printf '%s' "${PASSWORD}" | sudo -S systemctl restart ${service} 
  rm -f ${temp_file}
EOF
