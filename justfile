# Justfile for AI Social Network Simulation
# Run with: just <command>

# Default recipe - run all checks
default: check

# Run all checks (format, lint, typecheck, build)
check: format db-prepare lint typecheck build test

# Format all code
format:
    @echo "🎨 Formatting Python code..."
    uv run ruff format initializer simulator
    @echo "🎨 Formatting Rust code..."
    cd twitter-api-service && cargo fmt

# Lint all code
lint:
    @echo "🔍 Linting Python code..."
    uv run ruff check initializer simulator
    @echo "🔍 Linting Rust code..."
    cd twitter-api-service && SQLX_OFFLINE=true cargo clippy -- -D warnings

# Type check all code
typecheck:
    @echo "🔬 Type checking Python code..."
    uv run pyright
    @echo "🔬 Type checking Rust code..."
    cd twitter-api-service && SQLX_OFFLINE=true cargo check

# Build all projects
build:
    @echo "🔨 Building Python projects..."
    uv sync
    @echo "🔨 Building Rust project..."
    cd twitter-api-service && SQLX_OFFLINE=true cargo build

# Run tests
test:
    @echo "🧪 Running tests..."
    cd twitter-api-service && SQLX_OFFLINE=true cargo test

# Database operations
db-init:
    @echo "📦 Initializing database with schema..."
    psql "${DATABASE_URL?DATABASE_URL must be set}" -f twitter-api-service/schema.sql

db-prepare:
    @echo "💾 Preparing SQLx offline data..."
    cd twitter-api-service && DATABASE_URL="${DATABASE_URL?DATABASE_URL must be set}" cargo sqlx prepare

# Docker operations
docker-up:
    @echo "🐳 Starting Docker services..."
    docker compose up -d

docker-down:
    @echo "🐳 Stopping Docker services..."
    docker compose down

docker-rebuild:
    @echo "🐳 Rebuilding Docker services..."
    docker compose up --build -d

# Development workflow
dev: docker-up db-init check
    @echo "✅ Development environment ready!"


# Clean up build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    cd twitter-api-service && cargo clean
    rm -rf .venv/__pycache__
    find . -name "*.pyc" -delete
    find . -name "__pycache__" -type d -exec rm -rf {} +

# Initialize project (first time setup)
init:
    @echo "🚀 Initializing project..."
    uv sync
    just docker-up
    sleep 10
    just db-init
    just db-prepare
    @echo "✅ Project initialized! Copy .env.example to .env and add your API keys."

# Run the simulation
run-init:
    @echo "🤖 Running initializer..."
    uv run python initializer/main.py

run-sim:
    @echo "🎭 Running simulator..."
    uv run python simulator/main.py

# Quick fixes
fix:
    @echo "🔧 Auto-fixing issues..."
    uv run ruff check --fix initializer simulator
    cd twitter-api-service && cargo fmt
    just typecheck

# Show help
help:
    @echo "Available commands:"
    @echo "  check      - Run all checks (auto-generates .sqlx data)"
    @echo "  format     - Format all code"
    @echo "  lint       - Lint all code" 
    @echo "  typecheck  - Type check all code"
    @echo "  build      - Build all projects"
    @echo "  test       - Run tests"
    @echo "  fix        - Auto-fix formatting and linting issues"
    @echo ""
    @echo "Database:"
    @echo "  db-init    - Initialize database with schema"
    @echo "  db-prepare - Generate SQLx offline data (auto-run with check)"
    @echo ""
    @echo "Docker:"
    @echo "  docker-up  - Start Docker services"
    @echo "  docker-down - Stop Docker services"  
    @echo "  docker-rebuild - Rebuild Docker services"
    @echo ""
    @echo "Development:"
    @echo "  dev        - Set up development environment"
    @echo "  init       - Initialize project (first time)"
    @echo "  run-init   - Run initializer service"
    @echo "  run-sim    - Run simulator service"
    @echo "  clean      - Clean build artifacts"