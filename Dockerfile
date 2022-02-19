

FROM rustlang/rust:nightly-slim as builder
WORKDIR /app
ADD src/ src/
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

FROM debian:10-slim
WORKDIR /app
COPY  docker-entrypoint.sh docker-entrypoint.sh
COPY --from=builder /app/target/* ./
RUN chmod a+x maga
RUN chmod a+x maga-serve
RUN chmod a+x docker-entrypoint.sh
ENTRYPOINT [ "/app/docker-entrypoint.sh" ]
