

FROM rust:latest AS build


ARG TARGET=aarch64-unknown-linux-musl

RUN rustup target add $TARGET
RUN apt update && apt install -y musl-tools musl-dev libssl-dev
RUN update-ca-certificates


WORKDIR /usr/src/par-acquit-de

COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo  build --target $TARGET --release \
    && rm -rf src

COPY . ./

RUN cargo install --target $TARGET --path . \
    && cp ./target/$TARGET/release/par-acquit-de . \
    && rm -rf target

CMD ["./par-acquit-de"]


FROM alpine

WORKDIR app

COPY --from=build /usr/src/par-acquit-de/par-acquit-de .
COPY --from=build /usr/src/par-acquit-de/public ./public
COPY --from=build /usr/src/par-acquit-de/templates ./templates

COPY data.base.csv data.csv




CMD ["./par-acquit-de"]

