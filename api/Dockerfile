FROM rust:1.89-slim-bookworm as builder

WORKDIR /usr/src/app

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./.sqlx ./.sqlx

RUN mkdir src && echo "fn main() {}" > src/main.rs
ENV SQLX_OFFLINE=true
RUN cargo build --release

COPY ./src ./src

RUN rm -f target/release/deps/shordly_api*
RUN cargo build --release

RUN cargo install sqlx-cli --no-default-features --features postgres

FROM debian:bookworm-slim as final

ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq-dev postgresql-client \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /usr/src/app/target/release/shordly_api ${APP}/shordly_api

COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

COPY ./migrations ${APP}/migrations

COPY entrypoint.sh ${APP}/entrypoint.sh
RUN chmod +x ${APP}/entrypoint.sh

RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}

EXPOSE 3000

CMD ["./entrypoint.sh"]
