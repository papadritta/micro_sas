# Build stage
FROM rust:1.67 AS build
WORKDIR /app
COPY . .
RUN cargo build --release

# Distroless runtime stage
FROM gcr.io/distroless/cc-debian11
COPY --from=build /app/target/release/rust-axum-greedy-coin-microservice /app/

# Use non-root user
USER nonroot:nonroot

# Set up app directory
ENV APP_HOME=/app
WORKDIR $APP_HOME

# Expose port
EXPOSE 3000

# Run app
ENTRYPOINT ["/app/rust-axum-greedy-coin-microservice"]