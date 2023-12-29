FROM lukemathwalker/cargo-chef:latest-rust-1.73-buster AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y
# RUN cargo install --version=0.7.2 sqlx-cli \
#   --no-default-features --features postgres

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Just building deps
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE true
# ARG DATABASE_URL
# RUN sqlx database create --database-url=$DATABASE_URL; \
#   sqlx migrate run --database-url=$DATABASE_URL
RUN cargo build --release --bin newsletter
RUN cargo install livejq
RUN echo `whereis livejq`

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates jq \
  # Clean up
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/newsletter newsletter
COPY --from=builder /usr/local/cargo/bin/livejq /usr/bin/livejq
COPY config config
ENV APP_ENVIRONMENT production

ENTRYPOINT ["bash", "-c", "./newsletter | livejq"]
