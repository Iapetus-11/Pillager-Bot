FROM rust:1.95-alpine3.23 AS build

WORKDIR /pillager-bot

RUN apk update
RUN apk add postgresql-dev

COPY Cargo.lock Cargo.toml ./

COPY src/ ./src/
COPY migrations/ ./migrations/
COPY .sqlx ./.sqlx/

# TODO: Cache dependencies in separate layer

ENV SQLX_OFFLINE=true
RUN cargo build --locked --profile release

FROM alpine:3.23 AS runner

RUN apk update
RUN apk add libpq

WORKDIR /pillager-bot

COPY --from=build /pillager-bot/target/release/Pillager-Bot .

CMD ["./Pillager-Bot"]
