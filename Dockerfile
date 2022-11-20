# Dockerfile for creating a statically-linked Rust application using docker's
# multi-stage build feature. This also leverages the docker build cache to avoid
# re-downloading dependencies if they have not changed.
FROM rust:latest AS build
WORKDIR /usr/src

# Download the target for static linking.
RUN rustup target add aarch64-unknown-linux-gnu

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new par-acquit-de
WORKDIR /usr/src/par-acquit-de
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the source and build the application.
COPY . ./
RUN cargo install --target aarch64-unknown-linux-gnu --path .

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=build /usr/local/cargo/bin/par-acquit-de .
USER 1000
CMD ["./par-acquit-de"]
