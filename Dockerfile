FROM rust:buster as build
 
# Create a new empty shell project
RUN USER=root cargo new --bin murgi
WORKDIR /murgi

# Copies manifest files from our machine to the Image
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Allocate a Git Hash to the build
ARG GIT_HASH
ENV GIT_HASH=$(GIT_HASH:-dev) \
    PKG_CONFIG_PATH="/usr/lib/arm-linux-gnueabihf/pkgconfig" \
    PATH=/cargo/bin:$PATH \
    PKG_CONFIG_ALLOW_CROSS="true"

# Add armv7 architecture
RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install -y curl git build-essential && \
    apt-get install -y libc6-armhf-cross libc6-dev-armhf-cross gcc-arm-linux-gnueabihf && \
    rustup target add x86_64-unknown-linux-gnu && \
    rustup target add armv7-unknown-linux-gnueabihf 


# Build only dependencies to cache them
RUN cargo build --release && \
    rm src/*.rs

# After dependencies are built, copy the source code
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/murgi_bot* && \
    cargo build --release

# Final base for running the application
FROM debian:buster-slim

# Copy the build artifact from the build stage
COPY --from=build /murgi/target/release/murgi_bot .

CMD ["./murgi_bot"]
