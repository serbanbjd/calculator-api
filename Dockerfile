# syntax=docker/dockerfile:1
# ADR-0006: multi-stage build, distroless non-root runtime.

FROM rust:1-slim-bookworm AS build
WORKDIR /app

# Build dependencies first so this layer is cached until Cargo.toml/Cargo.lock change.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo 'fn main() {}' > src/main.rs \
    && touch src/lib.rs \
    && cargo build --release --locked \
    && rm -rf src

COPY src ./src
# Make sure cargo rebuilds our crate, not the cached stub.
RUN touch src/main.rs src/lib.rs && cargo build --release --locked

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=build /app/target/release/calculator-api /calculator-api
ENV PORT=8080
EXPOSE 8080
USER nonroot
ENTRYPOINT ["/calculator-api"]
