# ---- Build stage ----
FROM rust:1.87.0-alpine3.21 AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

# Set work directory and copy source
WORKDIR /app
COPY . .

# Build the release binary
RUN cargo build --release

# ---- Runtime stage ----
FROM alpine:3.21

# Install runtime dependencies (if needed)
RUN apk add --no-cache libgcc

# Copy binary from builder
COPY --from=builder /app/target/release/api /usr/local/bin/api

# Set executable
CMD ["api"]
