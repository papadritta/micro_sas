# Build stage
FROM rust:1.84 AS build
WORKDIR /app

# Copy project files
COPY . .

# Add MUSL target for static linking
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

# Minimal runtime stage
FROM gcr.io/distroless/cc-debian11:latest

# Use non-root user
USER nonroot:nonroot

# Set up app directory
ENV APP_HOME=/app
WORKDIR $APP_HOME

# Copy the statically built binary
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/rust-axum-greedy-coin-microservice /app/

# Expose the application's port
EXPOSE 3000

# Run the application
ENTRYPOINT ["/app/rust-axum-greedy-coin-microservice"]
