# Justfile for AI Social Network Simulation
# Run with: just <command>

# Default recipe - run all checks
default: check

# Run all checks (format, lint, typecheck, build)
check: format db-prepare lint typecheck build test viz-check

# Format all code
format:
    @echo "🎨 Formatting Python code..."
    uv run ruff format initializer simulator
    @echo "🎨 Formatting Rust code..."
    cd twitter-api-service && cargo fmt
    @echo "🎨 Formatting TypeScript/Svelte code..."
    pnpm lint:fix

# Lint all code
lint:
    @echo "🔍 Linting Python code..."
    uv run ruff check initializer simulator
    @echo "🔍 Linting Rust code..."
    cd twitter-api-service && SQLX_OFFLINE=true cargo clippy -- -D warnings
    @echo "🔍 Linting TypeScript/Svelte code..."
    pnpm lint

# Type check all code
typecheck:
    @echo "🔬 Type checking Python code..."
    uv run pyright
    @echo "🔬 Type checking Rust code..."
    cd twitter-api-service && SQLX_OFFLINE=true cargo check
    @echo "🔬 Type checking TypeScript/Svelte code..."
    pnpm check

# Build all projects
build:
    @echo "🔨 Building Python projects..."
    uv sync
    @echo "🔨 Building Rust project..."
    cd twitter-api-service && SQLX_OFFLINE=true cargo build
    @echo "🔨 Building visualizer..."
    pnpm build

# Run tests
test:
    @echo "🧪 Running tests..."
    cd twitter-api-service && SQLX_OFFLINE=true cargo test

# Database operations (schema initialization handled by Docker)

db-prepare:
    @echo "💾 Preparing SQLx offline data (requires Docker services running)..."
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
dev: docker-up check
    @echo "✅ Development environment ready!"


# Clean up build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    cd twitter-api-service && cargo clean
    rm -rf .venv/__pycache__
    find . -name "*.pyc" -delete
    find . -name "__pycache__" -type d -exec rm -rf {} +
    rm -rf dist node_modules

# Initialize project (first time setup)
init:
    @echo "🚀 Initializing project..."
    uv sync
    pnpm install
    just docker-up
    sleep 10
    just db-prepare
    @echo "✅ Project initialized! Copy .env.example to .env and add your API keys."

# Run the simulation
run-init:
    @echo "🤖 Running initializer..."
    uv run python initializer/main.py

run-sim:
    @echo "🎭 Running simulator..."
    uv run python simulator/main.py

# Visualizer operations
viz-dev:
    @echo "🎨 Starting visualizer dev server..."
    pnpm dev

viz-build:
    @echo "🔨 Building visualizer for production..."
    pnpm build

viz-preview:
    @echo "👀 Previewing visualizer build..."
    pnpm preview

viz-check:
    @echo "✅ Running visualizer checks..."
    pnpm check

viz-lint:
    @echo "🔍 Linting visualizer..."
    pnpm lint

viz-install:
    @echo "📦 Installing visualizer dependencies..."
    pnpm install

# Quick fixes
fix:
    @echo "🔧 Auto-fixing issues..."
    uv run ruff check --fix initializer simulator
    cd twitter-api-service && cargo fmt
    pnpm lint:fix
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
    @echo "  db-prepare - Generate SQLx offline data (requires Docker services)"
    @echo ""
    @echo "Docker:"
    @echo "  docker-up  - Start Docker services"
    @echo "  docker-down - Stop Docker services"  
    @echo "  docker-rebuild - Rebuild Docker services"
    @echo ""
    @echo "Development:"
    @echo "  dev        - Set up development environment (Docker + checks)"
    @echo "  init       - Initialize project (first time setup)"
    @echo "  run-init   - Run AI agent initializer (requires Docker services)"
    @echo "  run-sim    - Run simulation (requires Docker services + agents)"
    @echo "  clean      - Clean build artifacts"
    @echo ""
    @echo "Visualizer:"
    @echo "  viz-dev    - Start visualizer development server"
    @echo "  viz-build  - Build visualizer for production"
    @echo "  viz-preview - Preview visualizer build"
    @echo "  viz-check  - Run visualizer type checking"
    @echo "  viz-lint   - Lint visualizer code"
    @echo "  viz-install - Install visualizer dependencies"