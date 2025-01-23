FROM rust:slim-bullseye AS build

WORKDIR /pillager-bot

RUN apt-get update
RUN apt-get install libpq-dev -y

COPY Cargo.lock Cargo.toml ./

COPY src/ ./src/
COPY migrations/ ./migrations/
COPY .sqlx ./.sqlx/

# TODO: Cache dependencies in separate layer

ENV SQLX_OFFLINE=true
RUN cargo build --locked --profile release

FROM debian:bullseye-slim AS runner

RUN apt-get update
RUN apt-get install libpq5 -y

WORKDIR /pillager-bot

COPY --from=build /pillager-bot/target/release/Pillager-Bot .

CMD ["./Pillager-Bot"]
