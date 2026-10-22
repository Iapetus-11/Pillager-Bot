FROM rust:1.99.1-alpine3.24 AS build

WORKDIR /pillager-bot

RUN apk update
RUN apk add postgresql-dev openssl-dev openssl-libs-static pkgconf

COPY Cargo.lock Cargo.toml ./

COPY src/ ./src/
COPY migrations/ ./migrations/
COPY .sqlx ./.sqlx/

# TODO: Cache dependencies in separate layer

ENV SQLX_OFFLINE=true
RUN cargo build --locked --profile release

FROM alpine:3.24 AS runner

RUN apk update
RUN apk add libpq libssl3 ca-certificates

WORKDIR /pillager-bot

COPY --from=build /pillager-bot/target/release/Pillager-Bot .

CMD ["./Pillager-Bot"]
