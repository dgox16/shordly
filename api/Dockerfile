FROM rust:1.89-slim-trixie AS builder

WORKDIR /usr/src/app

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations

RUN mkdir src && echo "fn main() {}" > src/main.rs
ENV SQLX_OFFLINE=true
RUN cargo build --release

RUN rm -f target/release/app
COPY ./src ./src

RUN rm -f target/release/deps/shordly_api*
RUN cargo build --release
RUN strip target/release/shordly_api

FROM gcr.io/distroless/base-debian12:nonroot

WORKDIR /app

COPY --from=builder /lib/x86_64-linux-gnu/libgcc_s.so.1 /lib/x86_64-linux-gnu/
COPY --from=builder --chown=nonroot:nonroot /usr/src/app/migrations ./migrations
COPY --from=builder /usr/src/app/target/release/shordly_api .

EXPOSE 3000

CMD ["./shordly_api"]
