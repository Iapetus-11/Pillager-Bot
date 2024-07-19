# FROM rust:alpine3.18 AS build

# WORKDIR /pillager-bot

# COPY Cargo.lock Cargo.toml diesel.toml ./
# COPY src/ ./src/

# RUN apk add musl musl-dev libpq-dev postgresql-dev openssl
# # 

# RUN cargo install --path .

# FROM alpine:3.18 AS runner

# WORKDIR /pillager-bot

# COPY --from=build ~/.cargo/bin/Pillager-Bot .

# CMD ["Pillager-Bot"]

FROM rust:slim-bullseye AS build

WORKDIR /pillager-bot

RUN apt-get update
RUN apt-get install libpq-dev -y

COPY Cargo.lock Cargo.toml diesel.toml ./
COPY src/ ./src/

RUN cargo install --path . --root /pillager-bot/build/

FROM rust:slim-bullseye AS runner

RUN apt-get update
RUN apt-get install libpq5 -y

WORKDIR /pillager-bot

COPY --from=build /pillager-bot/build/bin/Pillager-Bot .

CMD ["./Pillager-Bot"]
