ARG RUST_VERSION=1.77
FROM rust:${RUST_VERSION}-buster AS dependency

WORKDIR /opt/glycoflow 

RUN mkdir -p src && echo "fn main() {}" >> src/main.rs

COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo fetch

FROM dependency AS build

COPY . .
RUN --mount=type=cache,target=/opt/target/ \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml  \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock  \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/glycoflow /bin/server

FROM debian:bullseye-slim AS final

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "1000" \
    appuser
USER appuser

COPY --from=build /bin/server /bin/

EXPOSE 8080

ENTRYPOINT [ "/bin/server" ]
