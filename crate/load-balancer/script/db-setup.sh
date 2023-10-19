#!/bin/bash
# This script updates a remote PostgreSQL database using Diesel CLI.

# Usage help
if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
  printf "Usage: $0 [-h/c]\n"
  printf "  --create / -c\tcreate db\n"
  printf "Environment Variables:\n"
  printf "  DATABASE_URL\postgresql url to exist db example: postgres://user:password@host/dbname\n"
  exit 0
fi

# Check if DATABASE_URL is provided
echo "Check env's"
if [ -z "${DATABASE_URL}" ]; then
  echo "Error: DATABASE_URL is required."
  exit 1
fi

# Usage help
if [ "$1" == "--create" ] || [ "$1" == "-c" ]; then
  # Extract information from URL
  user=$(echo "${DATABASE_URL}" | sed -n 's|^.*//\([^:]*\):.*$|\1|p')
  password=$(echo "${DATABASE_URL}" | sed -n 's|^.*:\([^@]*\)@.*$|\1|p')
  host_port=$(echo "${DATABASE_URL}" | sed -n 's|postgres://[^@]*@\([^/]*\).*|\1|p')
  host=$(echo "${host_port}" | cut -d ':' -f 1)
  port=$(echo "${host_port}" | cut -d ':' -f 2)
  db_name=$(echo "${DATABASE_URL}" | sed -n 's|.*/\([^/]*\)$|\1|p')

  # Create new database
  env PGPASSWORD="${password}" createdb -h "${host}" -p "${port}" -U "${user}" "${db_name}"
fi

dir="$(dirname "$(realpath "$0")")/"
cd "${dir}../"

# Run Diesel migration
echo "Make migrations"
diesel migration run --migration-dir "./migration"
if [ $? -ne 0 ]; then
  echo "Diesel migration failed."
  exit 1
fi

echo "Database updated successfully."