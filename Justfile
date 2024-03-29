set dotenv-load := true

DB_URL    := env_var('DATABASE_URL')
DB_USER   := env_var('DB_USER')
DB_PSWD   := env_var('DB_PSWD')
DB_NAME   := env_var('DB_NAME')
DB_PORT   := env_var('DOCKER_DB_PORT')
APP_PORT  := env_var('DOCKER_APP_PORT')
RMQ_PORT  := env_var('DOCKER_RMQ_UI_PORT')

test:
  @curl -v http://localhost:{{APP_PORT}}/health_check

sub email name port=(APP_PORT):
  @payload=$(python -c "from urllib.parse import quote; \
    print(f'email={quote(\"{{email}}\")}&name={quote(\"{{name}}\")}')");\
  curl -i -k --http3 -X POST -d "$payload" \
    http://localhost:{{port}}/subscriptions

psql:
  @export PGPASSFILE=".pgpass_db";\
  psql -h localhost -U {{DB_USER}} -p {{DB_PORT}} -d {{DB_NAME}}

migrate:
  sqlx database create --database-url={{DB_URL}}
  sqlx migrate run --database-url={{DB_URL}}

up context="desktop-linux":
  docker --context {{context}} compose -f compose.yaml up -d --build

drop_mockdb:
  @export PGPASSFILE=".pgpass_pg";\
  PSQL="psql -h localhost -U {{DB_USER}} -p {{DB_PORT}} -d postgres";\
  DATABASES_TO_DROP=$($PSQL -t \
    -c "SELECT datname FROM pg_database WHERE datname NOT IN ('newsletter', 'postgres', 'template1', 'template0');");\
  for db_name in $DATABASES_TO_DROP; do \
    printf "Database ID: $db_name -- ";\
    $PSQL -c "DROP DATABASE IF EXISTS \"$db_name\";";\
  done 

wipe_table table="subscriptions":
  @export PGPASSFILE=".pgpass_db";\
  PSQL="psql -h localhost -U {{DB_USER}} -p {{DB_PORT}} -d {{DB_NAME}}";\
  printf "Truncating table: {{table}} -- ";\
  $PSQL -c "TRUNCATE TABLE \"{{table}}\" RESTART IDENTITY CASCADE;"

show_data table="subscriptions":
  @export PGPASSFILE=".pgpass_db";\
  PSQL="psql -h localhost -U {{DB_USER}} -p {{DB_PORT}} -d {{DB_NAME}}";\
  $PSQL -c "SELECT email, name, subscribed_at FROM {{table}};"

ngrok:
  ngrok http {{APP_PORT}}

rabbitmq:
  firefox http://localhost:{{RMQ_PORT}}

prepare:
  cd server_rs; cargo sqlx prepare -D {{DB_URL}} -- --all-targets --all-features

watch:
  cd server_rs;\
  SQLX_OFFLINE=true cargo watch -x check -x 'test -- --nocapture'
