#!/bin/bash

export PGPASSFILE=".pgpass_db"

# Connect to the PostgreSQL server
PSQL_COMMAND="psql -h localhost -p 5434 -U danescher98 -d newsletter"

# Truncate each table in a loop
$table_name="subscriptions"
printf "Truncating table: $table_name -- "
$PSQL_COMMAND -c "TRUNCATE TABLE \"$table_name\" RESTART IDENTITY CASCADE;"
