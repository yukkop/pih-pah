#!/bin/bash
# This script updates a remote PostgreSQL database using Diesel CLI.

# Usage help
if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
  printf "Usage: $0 [-h]\n"
  printf "Environment Variables:\n"
  printf "  DATABASE_URL\postgresql url to exist db example: postgres://user:password@host/dbname\n"
  printf "  DB_NAME\db name that will create (default: pih-pah)\n"
  exit 0
fi

default_db_name="pih-pah"

if [ -z "${DB_NAME}" ]; then
  DB_NAME="${default_db_name}"
fi

# Check if DATABASE_URL is provided
echo "Check env's"
if [ -z "${DATABASE_URL}" ]; then
  echo "Error: DATABASE_URL is required."
  exit 1
fi

new_db_url="$(echo "${DATABASE_URL}" | sed "s|[^/]*$|${DB_NAME}|")"

# Run query
echo "Check db exist"
psql -tAc "SELECT 1 FROM pg_database WHERE datname='${DB_NAME}';" "${DATABASE_URL}" # > /dev/null 2>&1
# Create database if it doesn't exist
if [ $? -ne 0 ]; then
  echo "creating db"
  psql -c "CREATE DATABASE \"${DB_NAME}\";" "${DATABASE_URL}"
  if [ $? -ne 0 ]; then
    echo "Database creation failed."
    exit 1
  fi
fi

# Export DATABASE_URL for Diesel CLI
export DATABASE_URL="${new_db_url}"

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