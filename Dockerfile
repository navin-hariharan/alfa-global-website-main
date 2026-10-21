# syntax=docker/dockerfile:1

# ---- Build -----------------------------------------------------------------
FROM rust:1-slim-bookworm AS build
WORKDIR /app

# Cache dependencies separately from the site's own code.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
 && cargo build --release --locked \
 && rm -rf src target/release/deps/alfa_site* target/release/alfa-site*

# Templates are compiled into the binary, so both folders are needed here.
COPY src src
COPY templates templates
RUN cargo build --release --locked \
 && mkdir -p /out/data

# ---- Runtime ---------------------------------------------------------------
# Distroless: no shell or package manager, runs as an unprivileged user.
FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app

COPY --from=build /app/target/release/alfa-site /app/alfa-site
COPY static /app/static
COPY --from=build --chown=65532:65532 /out/data /data

ENV PORT=8080 \
    STATIC_DIR=/app/static \
    ENQUIRIES_FILE=/data/enquiries.jsonl \
    RUST_LOG=alfa_site=info,tower_http=warn

EXPOSE 8080
VOLUME ["/data"]
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 CMD ["/app/alfa-site", "healthcheck"]
ENTRYPOINT ["/app/alfa-site"]
