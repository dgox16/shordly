FROM rust:1.79.0 as builder

RUN USER=root cargo new --bin shordly_api
WORKDIR ./shordly_api
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
COPY ./Cargo.toml ./Cargo.toml
COPY ./.sqlx/ /.sqlx/

ENV SQLX_OFFLINE=true
RUN cargo build --release

RUN rm src/*.rs
ADD . ./
RUN rm ./target/release/deps/shordly_api*
RUN cargo build --release

FROM debian:bookworm-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 3000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /shordly_api/target/release/shordly_api ${APP}/shordly_api

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./shordly_api"]
