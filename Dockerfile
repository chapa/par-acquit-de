FROM rust:latest as builder
WORKDIR /usr/src/par-acquit-de
COPY . .

RUN ls -laR
RUN cargo install --path .
FROM debian:bullseye-slim
RUN uname -a
RUN apt-get update & apt-get install -y open-ssl extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/par-acquit-de /usr/local/bin/par-acquit-de
WORKDIR /app

COPY --from=builder /usr/src/par-acquit-de/public /app/public
COPY --from=builder /usr/src/par-acquit-de/templates /app/templates

CMD ["par-acquit-de"]
