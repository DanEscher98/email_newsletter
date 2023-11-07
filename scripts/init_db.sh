#!/usr/bin/env bash

# Debug mode
# set -x
set -o pipefail

required_bins=("psql" "sqlx")
for bin_name in "${required_bins[@]}"; do
    if ! [ -x "$(command -v $bin_name)" ]; then
        echo >&2 "Error: $bin_name is not installed."
        exit 1
    fi
done

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRRES_PORT:=5432}"

function create_container() {
  echo "$(podman create \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    postgres -N 1000)"
}

if [ -s .container_id ]; then
  CONTAINER_ID="$(cat .container_id)"
  CREATED=$(podman inspect "$CONTAINER_ID" 2>/dev/null | jq '.[0].Created')
  if [ $? -eq 0 ]; then
    echo "Using existing container: $CREATED"
    echo "ID: $CONTAINER_ID"
  else
    echo "Creating new container"
    CONTAINER_ID=$(create_container)
    echo "ID: $CONTAINER_ID"
    echo "$CONTAINER_ID" > .container_id
  fi
else
  echo "Creating new container"
  CONTAINER_ID=$(create_container)
  echo "ID: $CONTAINER_ID"
  echo "$CONTAINER_ID" > .container_id
fi

PID=$(lsof -ti ":$DB_PORT")
if [ $? -eq 0 ]; then
  echo "Killing existing process: $PID"
  kill -0 "$PID"
fi
podman start $CONTAINER_ID 2>/dev/null
RUNNING=$(podman inspect $CONTAINER_ID | jq '.[0].State.Running')

if [ "$RUNNING" = "true" ]; then
  # Keep pinging Postgres until it's ready to accept commands
  export PGPASSWORD="${DB_PASSWORD}"
  until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
  done
  >&2 echo "Postgres is up and running on port ${DB_PORT}!"
else
  printf "\nContainer State: "
  podman inspect "$CONTAINER_ID" 2>/dev/null | jq --tab -C '.[0].State' >/dev/tty
fi

DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"
echo "DATABASE_URL: $DATABASE_URL"
sqlx database create --database-url="$DATABASE_URL"
sqlx migrate run --database-url="$DATABASE_URL"

