APP_PORT  := `yq '.application.port' config/base.yaml`
APP_HOST  := `yq '.application.host' config/local.yaml`

DB_HOST   := `yq '.database.host' config/base.yaml`
DB_PORT   := `yq '.database.port' config/base.yaml`
DB_NAME   := `yq '.database.database_name' config/base.yaml`
DB_USER   := `yq '.database.username' config/base.yaml`
DB_PSWD   := `yq '.database.password' config/base.yaml`

test_db:
  @curl -sSX GET http://{{APP_HOST}}:{{APP_PORT}}/test_db/ | jq

pg_url: 
  @echo "postgres://{{DB_USER}}:{{DB_PSWD}}@localhost:5434/{{DB_NAME}}"

psql:
  @psql -h localhost -U {{DB_USER}} -p 5434 -d {{DB_NAME}} -W

migrate:
  sqlx database create --database-url="$(just pg_url)"
  sqlx migrate run --database-url="$(just pg_url)"

build:
  @# docker compose -f compose.yaml down
  docker compose -f compose.yaml up -d --build
