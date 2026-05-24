# Hotel Booking Backend

A REST API for managing hotels and room types, built with Rust and Actix Web. Authentication is handled via OpenID Connect (OAuth 2.0 with PKCE) through Keycloak.

## Tech Stack

| Layer | Technology |
|-------|------------|
| Language | [Rust](https://www.rust-lang.org/) (2024 edition) |
| Web framework | [Actix Web](https://actix.rs/) |
| Database | [PostgreSQL 18](https://www.postgresql.org/) |
| Database access | [SQLx](https://github.com/launchbadge/sqlx) (async, compile-time checked queries) |
| Authentication | [Keycloak](https://www.keycloak.org/) + [oauth2](https://github.com/ramosbugs/oauth2-rs) (PKCE flow) |
| Sessions | [actix-session](https://docs.rs/actix-session/) (cookie-based, `hotel-booking` cookie) |
| HTTP client | [reqwest](https://docs.rs/reqwest/) |
| API documentation | [Utoipa](https://github.com/juhaku/utoipa) + [Swagger UI](https://swagger.io/tools/swagger-ui/) |
| Configuration | [config](https://github.com/mehcode/config-rs) (`src/appsettings.json`) |
| Serialization | [Serde](https://serde.rs/) + [rust_decimal](https://docs.rs/rust_decimal/) |
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

### 1. Start Infrastructure

From the project root, start PostgreSQL and Keycloak:

```bash
docker compose up -d
```

This starts three services:

| Service | URL / Port | Purpose |
|---------|------------|---------|
| PostgreSQL (app) | `localhost:5432` | Application database |
| PostgreSQL (Keycloak) | `localhost:5433` | Keycloak database |
| Keycloak | http://localhost:8180 | Identity provider |

Keycloak admin console: http://localhost:8180 (default credentials: `admin` / `admin`).

### 2. Configure Keycloak Client

Create an OpenID Connect client in Keycloak:

1. Open the Keycloak admin console and select the **master** realm.
2. Go to **Clients** → **Create client**.
3. Set **Client ID** to `hotel-booking`.
4. Enable **Client authentication** and copy the generated **Client secret**.
5. Set **Valid redirect URIs** to `http://localhost:8080/api/authentication/callback`.
6. Save the client.

### 3. Configure Application Settings

Update `src/appsettings.json` with your values.

```json
{
  "ConnectionStrings": {
    "Database": "postgresql://postgres:postgres@localhost:5432/hotel-booking-db"
  },
  "Authentication": {
    "ClientId": "",
    "ClientSecret": "",
    "TenantId": "",
    "BaseUrl": "http://localhost:8180",
    "RedirectUrl": "http://localhost:8080/api/authentication/callback"
  }
}
```

Fill in `ClientId`, `ClientSecret`, and `TenantId` after creating the Keycloak client (step 2). Keep `RedirectUrl` in sync with the redirect URI registered in Keycloak.

### 4. Run Database Migrations

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

### 5. Build and Run the API

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

### 6. Explore the API (Swagger UI)

Open Swagger UI in your browser:

**http://127.0.0.1:8080/swagger/index.html**

The OpenAPI spec is also available at **http://127.0.0.1:8080/api-docs/openapi.json**.

![Swagger UI — Hotel Booking API endpoints](docs/swagger-ui.png)

## Authentication Flow

1. `GET /api/authentication/login` — generates a PKCE challenge, stores the verifier and CSRF token in a session cookie, and redirects the browser to Keycloak.
2. The user signs in on Keycloak, which redirects back to `/api/authentication/callback?code=...&state=...`.
3. `GET /api/authentication/callback` — validates the CSRF token, exchanges the authorization code for tokens, and stores the access token in the session.

To test locally, open http://127.0.0.1:8080/api/authentication/login in a browser while the API and Keycloak are running.

## API Endpoints

Endpoints are grouped in Swagger UI under three tags: **Authentication**, **Hotels**, and **Room Types**.

### Authentication

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/authentication/login` | Redirects to Keycloak for sign-in |
| `GET` | `/api/authentication/callback` | OAuth callback; exchanges code for tokens and stores access token in session |

### Hotels

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/hotels` | List hotels (optional filters: `names`, `from_rating`, `to_rating`) |
| `POST` | `/api/hotels` | Create a new hotel |

### Room Types

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/hotels/{id}/room-type` | Add a room type to a hotel |

Sample HTTP requests for hotel and room type endpoints are in the `http/` directory.

## Project Structure

```
src/
├── main.rs                  # App entry point, OpenAPI & Swagger UI setup
├── appsettings.json         # Database and authentication settings
├── domain/                  # Domain models and errors
├── endpoints/
│   ├── authentication/      # Login and OAuth callback handlers
│   ├── hotels/              # Hotel endpoints
│   └── room_types/          # Room type endpoints
├── middlewares/             # Cookie session middleware
├── persistence/
│   ├── migrations/          # SQLx migration files
│   └── repositories/        # Database access layer
└── providers/
    ├── auth_provider.rs     # OAuth2 / Keycloak client
    ├── config_provider.rs   # appsettings.json loader
    ├── pg_provider.rs       # PostgreSQL connection pool
    └── common/              # Shared provider errors
```

## Configuration

All runtime settings live in `src/appsettings.json`.

### ConnectionStrings

| Key | Description | Default |
|-----|-------------|---------|
| `Database` | PostgreSQL connection string for the application database | `postgresql://postgres:postgres@localhost:5432/hotel-booking-db` |

Update this if you change Docker Compose credentials or use a different host/port.

### Authentication

| Key | Description | Placeholder / default |
|-----|-------------|----------------------|
| `ClientId` | Keycloak client ID | `""` — set to your client ID, e.g. `"hotel-booking"` |
| `ClientSecret` | Keycloak client secret | `""` — set to the secret from the Keycloak client |
| `TenantId` | Keycloak realm name | `""` — set to your realm, e.g. `"master"` |
| `BaseUrl` | Keycloak server URL | `"http://localhost:8180"` |
| `RedirectUrl` | OAuth callback URL registered in Keycloak | `"http://localhost:8080/api/authentication/callback"` |

Example after configuration:

```json
{
  "ConnectionStrings": {
    "Database": "postgresql://postgres:postgres@localhost:5432/hotel-booking-db"
  },
  "Authentication": {
    "ClientId": "hotel-booking",
    "ClientSecret": "your-keycloak-client-secret",
    "TenantId": "master",
    "BaseUrl": "http://localhost:8180",
    "RedirectUrl": "http://localhost:8080/api/authentication/callback"
  }
}
```
