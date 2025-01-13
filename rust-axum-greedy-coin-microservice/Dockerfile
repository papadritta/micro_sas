# Build stage
FROM rust:1.84 AS build
WORKDIR /app
COPY . .
RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && apt-get install -y musl-tools && \
    cargo build --release --target x86_64-unknown-linux-musl

# Distroless runtime stage
FROM gcr.io/distroless/cc-debian11
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/rust-axum-greedy-coin-microservice /app/

# Use non-root user
USER nonroot:nonroot

# Set up app directory
ENV APP_HOME=/app
WORKDIR $APP_HOME

# Expose port
EXPOSE 3000

# Run app
ENTRYPOINT ["/app/rust-axum-greedy-coin-microservice"]
