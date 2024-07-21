FROM rust:slim-bullseye AS build

WORKDIR /pillager-bot

RUN apt-get update
RUN apt-get install libpq-dev -y

COPY Cargo.lock Cargo.toml diesel.toml ./

# Install dependencies, but build a dummy project to cache deps separately from project files to avoid
# unnecessarily download+building dependencies
RUN mkdir src && echo '// dummy file\nfn main() {}' > ./src/main.rs
RUN cargo build

COPY src/ ./src/

RUN cargo install --path . --root ./build/

FROM rust:slim-bullseye AS runner

RUN apt-get update
RUN apt-get install libpq5 -y

WORKDIR /pillager-bot

COPY --from=build /pillager-bot/build/bin/Pillager-Bot .

CMD ["./Pillager-Bot"]
