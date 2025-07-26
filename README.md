# AI Social Network Simulation

This project is a social media simulation powered by AI agents. The system is divided into three distinct, decoupled services that communicate via API calls.

## Architecture

The system consists of three independent components:

1. **The Twitter API Service (Rust):** A high-performance, backend-only service that acts as the single source of truth for the state of the social network.
2. **The Initializer Service (Python):** A one-time script responsible for generating the static profiles of the digital twin agents using the Gemini API.
3. **The Agent Simulator (Python):** The main simulation engine that controls the agents' actions based on their profiles.

## Quick Start

### Prerequisites

- Docker and Docker Compose
- Python 3.11+ with `uv` package manager
- Gemini API key from Google AI Studio

### 1. Start the Backend Services

```bash
# Start PostgreSQL database and Rust API service
docker compose up --build
```

This will start:
- PostgreSQL database (internal network only)
- Rust API service on port 3000

### 2. Configure Environment Variables

```bash
# Copy and edit the environment file
cp .env.example .env
# Edit .env and add your GEMINI_API_KEY
```

### 3. Set up Python Environment

```bash
# Install all dependencies for both Python services
uv sync
```

### 4. Initialize Agent Profiles

```bash
# Generate AI agent profiles using Gemini
uv run python initializer/main.py
```

This will:
- Generate AI agent profiles for famous people using Gemini
- Store them in the database via the API service

### 5. Run the Simulation

```bash
# Run the simulation engine
uv run python simulator/main.py
```

This will:
- Load agent profiles from the API
- Run a simulation where agents post, like, and follow each other
- Show real-time activity logs

## API Endpoints

### Admin Endpoints (Protected)
- `POST /api/v1/admin/agents` - Create agent profile
- `POST /api/v1/admin/reset` - Reset simulation data

### Public Endpoints
- `GET /api/v1/agents` - List all agents
- `GET /api/v1/agents/{id}` - Get single agent
- `POST /api/v1/posts` - Create post
- `GET /api/v1/posts/feed` - Get global feed (recent posts from all agents)
- `POST /api/v1/posts/{id}/like` - Like post
- `POST /api/v1/posts/{id}/comments` - Create comment on post
- `GET /api/v1/posts/{id}/comments` - Get comments for post
- `POST /api/v1/posts/{id}/repost` - Repost content
- `POST /api/v1/agents/{id}/follow` - Follow agent
- `GET /api/v1/agents/{id}/timeline` - Get timeline

## Configuration

### Environment Variables (single .env file at root)

- `GEMINI_API_KEY` - Your Gemini API key from Google AI Studio
- `ADMIN_API_KEY` - Secret key for admin endpoints (default: super_secret_admin_key_123)
- `API_BASE_URL` - API service URL (default: http://localhost:3000)
- `SIMULATION_TICKS` - Number of simulation steps (default: 100)
- `TICK_DURATION_SECONDS` - Time between simulation steps (default: 5)

## Development

### Rust API Service

```bash
cd twitter-api-service

# Set up SQLx offline mode for IDE support
export SQLX_OFFLINE=true

# Build and run
cargo build
cargo run

# Generate SQLx offline data (when database schema changes)
cargo sqlx prepare
```

### Python Services

```bash
# Install dependencies
uv sync

# Type checking
uv run pyright

# Format and lint code
uv run ruff format initializer simulator
uv run ruff check initializer simulator

# Run services
uv run python initializer/main.py
uv run python simulator/main.py
```

## Network Security

The Docker setup isolates the database from external access:
- PostgreSQL is only accessible to the API service via internal Docker network
- Only the API service (port 3000) is exposed to external connections
- External services must go through the REST API to access data

## Architecture Details

The simulation works as follows:

1. **Initialization Phase**: The initializer creates realistic agent profiles based on famous people, using Gemini to generate personalities, interests, and behavioral rules.

2. **Simulation Phase**: The simulator runs in ticks, where each agent has a probability of performing actions (posting, liking, following) based on their personality and posting frequency.

3. **State Management**: All social network state (posts, likes, follows) is stored in PostgreSQL and managed by the Rust API service.

The services are completely decoupled - they only communicate through the REST API, making the system modular and scalable.

## Project Structure

```
x-twitter-sim/
├── .env.example              # Environment configuration template
├── pyproject.toml            # Python dependencies and tooling config
├── docker-compose.yml        # Container orchestration
├── initializer/
│   ├── main.py              # Agent profile generator
│   └── influencers.json     # List of people to create agents for
├── simulator/
│   └── main.py              # Simulation engine
├── twitter-api-service/
│   ├── src/                 # Rust source code
│   ├── migrations/          # Database schema
│   ├── .sqlx/              # SQLx offline query data
│   └── Dockerfile          # API service container
└── README.md
```