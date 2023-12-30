set dotenv-load := true

DB_URL    := env_var('DATABASE_URL')
DB_USER   := env_var('DB_USER')
DB_PSWD   := env_var('DB_PSWD')
DB_NAME   := env_var('DB_NAME')
DB_PORT   := env_var('DOCKER_DB_PORT')
APP_PORT  := env_var('DOCKER_APP_PORT')

test:
  @curl -v http://localhost:{{APP_PORT}}/health_check

sub email name:
  curl -i -k --http3 -X POST -d 'email={{email}}&name={{name}}' \
    http://localhost:{{APP_PORT}}/subscriptions

psql:
  @psql -h localhost -U {{DB_USER}} -p {{DB_PORT}} -d {{DB_NAME}} -W

migrate:
  sqlx database create --database-url={{DB_URL}}
  sqlx migrate run --database-url={{DB_URL}}

build:
  @# docker compose -f compose.yaml down
  docker compose -f compose.yaml up -d --build
