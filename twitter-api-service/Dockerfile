FROM rust:latest AS builder

WORKDIR /usr/src/app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release && rm src/main.rs

# Copy source and prepare offline sqlx
COPY src ./src
COPY .sqlx ./.sqlx

# Build the application in offline mode
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Runtime stage  
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /usr/src/app/target/release/twitter-api-service .

EXPOSE 3000

CMD ["./twitter-api-service"]