

FROM rust:latest AS build

ARG TARGET=aarch64-unknown-linux-musl

# Download the target for static linking.
RUN rustup target add $TARGET


WORKDIR /usr/src/par-acquit-de

COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo  build --target $TARGET --release \
    && rm -rf src

# Copy the source and build the application.
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

CMD ["./par-acquit-de"]
