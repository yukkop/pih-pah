
# Usage help
if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
  printf "Usage: $0 [-h] <name> <server address>\n"
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


name=$1;
address=$2;

id=$(uuidgen) > /dev/null 2>&1
if [ $? -ne 0 ]; then
 echo "but it is not problem!!"
 powershell -Command "[guid]::NewGuid().ToString()"
fi

psql "${DATABASE_URL}" -c <<EOF
INSERT INTO public."server"
(id, "name", country_id, online, address)
VALUES(${id}, "${name}", 1, false, ${address});
EOF