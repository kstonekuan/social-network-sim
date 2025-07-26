FROM rust:latest AS builder

WORKDIR /usr/src/app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy source and prepare offline sqlx
COPY src ./src
COPY .sqlx ./.sqlx

# Build the application in offline mode
ENV SQLX_OFFLINE=true
RUN cargo build

# Runtime stage  
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /usr/src/app/target/debug/twitter-api-service .

EXPOSE 3000

CMD ["./twitter-api-service"]