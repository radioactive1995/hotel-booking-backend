# Hotel Booking Backend

A REST API for managing hotels and room types, built with Rust and Actix Web.

## Tech Stack

| Layer | Technology |
|-------|------------|
| Language | [Rust](https://www.rust-lang.org/) (2024 edition) |
| Web framework | [Actix Web](https://actix.rs/) |
| Database | [PostgreSQL 18](https://www.postgresql.org/) |
| Database access | [SQLx](https://github.com/launchbadge/sqlx) (async, compile-time checked queries) |
| API documentation | [Utoipa](https://github.com/juhaku/utoipa) + [Swagger UI](https://swagger.io/tools/swagger-ui/) |
| Configuration | [config](https://github.com/mehcode/config-rs) (`src/appsettings.json`) |
| Serialization | [Serde](https://serde.rs/) |
| Containerization | [Docker Compose](https://docs.docker.com/compose/) |

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [Docker](https://www.docker.com/) and Docker Compose
- [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (for running migrations)

Install the SQLx CLI:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

## Getting Started

### 1. Start PostgreSQL

From the project root, start the database container:

```bash
docker compose up -d
```

This spins up PostgreSQL on port `5432` with the credentials defined in `docker-compose.yml` and `src/appsettings.json`.

### 2. Run Database Migrations

Set the database URL so SQLx can connect, then apply all pending migrations.

**Linux / macOS**

```bash
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/hotel-booking-db"
sqlx migrate run --source src/persistence/migrations
```

**Windows (PowerShell)**

```powershell
$env:DATABASE_URL = "postgresql://postgres:postgres@localhost:5432/hotel-booking-db"
sqlx migrate run --source src/persistence/migrations
```

> Migrations live in `src/persistence/migrations/`. The `--source` flag is required because they are not in SQLx's default `./migrations` directory.

### 3. Build and Run the API

The project uses SQLx compile-time query checking. Set `DATABASE_URL` before building so `cargo build` can verify queries against the live schema:

```bash
# Linux / macOS
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/hotel-booking-db"
cargo run
```

```powershell
# Windows (PowerShell)
$env:DATABASE_URL = "postgresql://postgres:postgres@localhost:5432/hotel-booking-db"
cargo run
```

The server listens on **http://127.0.0.1:8080**.

### 4. Explore the API (Swagger UI)

Open Swagger UI in your browser:

**http://127.0.0.1:8080/swagger/index.html**

The OpenAPI spec is also available at **http://127.0.0.1:8080/api-docs/openapi.json**.

<!-- TODO: Add a screenshot of the Swagger UI below -->

![Swagger UI — Hotel Booking API endpoints](docs/swagger-ui.png)

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/hotels` | List hotels (optional filters: `names`, `from_rating`, `to_rating`) |
| `POST` | `/api/hotels` | Create a new hotel |
| `POST` | `/api/hotels/{id}/room-type` | Add a room type to a hotel |

Sample HTTP requests are in the `http/` directory.

## Project Structure

```
src/
├── main.rs                  # App entry point, OpenAPI & Swagger UI setup
├── appsettings.json         # Database connection string
├── domain/                  # Domain models and errors
├── endpoints/               # HTTP handlers (hotels, room types)
├── persistence/
│   ├── migrations/          # SQLx migration files
│   └── repositories/        # Database access layer
└── providers/               # Config and PostgreSQL pool setup
```

## Configuration

Database connection settings are in `src/appsettings.json`:

```json
{
  "ConnectionStrings": {
    "Database": "postgresql://postgres:postgres@localhost:5432/hotel-booking-db"
  }
}
```

Update this if you change the Docker Compose credentials or use a different host/port.
