# AI Social Network Simulation

This project is a full-stack simulation of a social media platform like X (formerly Twitter). It includes a backend API, a frontend visualizer, and Python scripts to simulate user activity and generate data.

## Features

*   **Realistic Simulation:** Simulates user interactions like posting, following, and viewing timelines.
*   **Influencer Dynamics:** Models the behavior of influencers and their impact on the network.
*   **Data Visualization:** A Svelte-based frontend to visualize the social network graph and user activity in real-time.
*   **Containerized Services:** Uses Docker and Docker Compose for easy setup and consistent development environments.
*   **Comprehensive Tooling:** Leverages `just` for streamlined command execution for tasks like building, testing, and code formatting.

## Architecture

The project is composed of several key components:

*   **`twitter-api-service`**: A backend API built with Rust, Axum, and SQLx, responsible for handling all data persistence and business logic.
*   **`visualizer`**: A frontend application built with Svelte and Vite that consumes the backend API to display the simulation.
*   **`initializer`**: A Python script to set up the initial state of the database, creating users and influencers.
*   **`simulator`**: A Python script that runs the simulation by generating user actions (posts, follows, etc.) against the API.
*   **`PostgreSQL`**: The database used to store all application data, managed via Docker.

## Technology Stack

*   **Backend:** Rust, Axum, Tokio, SQLx
*   **Frontend:** Svelte, TypeScript, Vite, Tailwind CSS
*   **Simulation:** Python, Pydantic, httpx
*   **Database:** PostgreSQL
*   **Build/Automation:** Just, uv, pnpm
*   **Containerization:** Docker, Docker Compose

## Prerequisites

Before you begin, ensure you have the following installed:

*   [Docker](https://www.docker.com/get-started) - **Required** for all infrastructure (database, API)
*   [just](https://github.com/casey/just) - **Recommended** for convenient command execution
*   [uv](https://github.com/astral-sh/uv) - **Required** for Python simulation scripts
*   [pnpm](https://pnpm.io/installation) - **Optional** for frontend development only

## Getting Started

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd x-twitter-sim
    ```

2.  **Create the environment file:**
    Copy the example `.env.example` to `.env` and fill in the required values, such as your API keys and database connection string.
    ```bash
    cp .env.example .env
    ```

3.  **Run the initialization script:**
    This command will install dependencies, start Docker containers, and prepare the database.
    ```bash
    just init
    ```
    *Note: The database schema is automatically initialized by Docker. This might take a few minutes on the first run.*

## Development Workflow

Once the initial setup is complete, you can use the following commands to run and manage the application.

*   **Start all infrastructure:**
    This starts the PostgreSQL database and Rust API service with automatic schema initialization.
    ```bash
    just docker-up
    ```

*   **Run the AI agent initializer:**
    This populates the database with AI agent profiles using the Gemini API.
    ```bash
    just run-init
    ```

*   **Run the simulation:**
    This starts the simulation with AI agents interacting on the platform.
    ```bash
    just run-sim
    ```

*   **Start the frontend visualizer:**
    This will start the Svelte development server.
    ```bash
    just viz-dev
    ```
    You can now view the visualizer at `http://localhost:5173`.

## Available Commands

The `justfile` provides a convenient way to run common tasks. Here are some of the most useful commands:

| Command            | Description                                           |
| ------------------ | ----------------------------------------------------- |
| `just check`       | Run all checks (format, lint, typecheck, build, test) |
| `just format`      | Format all code (Python, Rust, TS/Svelte)             |
| `just lint`        | Lint all code                                         |
| `just typecheck`   | Type check all code                                   |
| `just build`       | Build all projects                                    |
| `just test`        | Run backend tests                                     |
| `just dev`         | Set up the full development environment               |
| `just docker-down` | Stop all Docker services                              |
| `just clean`       | Clean all build artifacts                             |

For a full list of commands, run `just help`.
