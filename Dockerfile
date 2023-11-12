# Start from a Rust base image
FROM rust:1.66 as builder

# Set the current working directory inside the container
WORKDIR /usr/src/microircd

# Copy over your manifest
COPY Cargo.toml .

# Copy your source tree
COPY src ./src

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Our second stage, that only gets the built binary
FROM debian:buster-slim
COPY --from=builder /usr/src/microircd/target/release/microircd /usr/local/bin/microircd

# Run the binary.
CMD ["microircd"]