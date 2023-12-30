#!/bin/bash

export PGPASSFILE=".pgpass_pg"

# Connect to the PostgreSQL server
PSQL_COMMAND="psql -h localhost -p 5434 -U danescher98 -d postgres"

# Get the list of databases to drop
DATABASES_TO_DROP=$($PSQL_COMMAND -t \
  -c "SELECT datname FROM pg_database WHERE datname NOT IN ('newsletter', 'postgres', 'template1', 'template0');")

# Drop each database in a loop
for db_name in $DATABASES_TO_DROP; do
  printf "Database ID: $db_name -- "
  $PSQL_COMMAND -c "DROP DATABASE IF EXISTS \"$db_name\";"
done
