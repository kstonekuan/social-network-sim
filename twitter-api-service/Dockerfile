# Build stage
FROM rust:latest AS builder

WORKDIR /usr/src/app

# Install sqlx-cli for migrations
RUN cargo install sqlx-cli --no-default-features --features postgres

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release && rm src/main.rs

# Copy source and schema
COPY src ./src
COPY schema.sql ./schema.sql

# Build (requires database connection now)
RUN cargo build --release

# Runtime stage  
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary and schema from builder
COPY --from=builder /usr/src/app/target/release/twitter-api-service .
COPY --from=builder /usr/src/app/schema.sql ./schema.sql
COPY --from=builder /usr/local/cargo/bin/sqlx ./

# Initialize database with schema then start app
CMD ["sh", "-c", "psql $DATABASE_URL -f schema.sql && ./twitter-api-service"]