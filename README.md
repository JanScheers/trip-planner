# China 2026 Trip Planner

21-day trip across 7 cities: Beijing → Xi'an → Chengdu → Chongqing → Zhangjiajie → Guilin → Hong Kong.

## Stack

- **Frontend:** Svelte 5 + TypeScript (Vite)
- **Backend:** Rust + Actix-Web REST API
- **Database:** SQLite (via sqlx)
- **Auth:** Google OAuth2 (whitelisted editors, public read)

## Getting Started

### 1. Backend

```bash
cd backend
cp .env .env.local   # edit with your Google OAuth credentials
cargo run
```

The server starts at `http://localhost:8080`. On first run it seeds the database from the `seed/` directory (`cities.tsv`, `accommodations.tsv`, `days.tsv`).

### 2. Frontend

```bash
cd frontend
npm install
npm run dev
```

The dev server starts at `http://localhost:5173` and proxies API calls to the backend.

## Configuration

Edit `backend/.env`:

| Variable               | Description                                     |
| ---------------------- | ----------------------------------------------- |
| `GOOGLE_CLIENT_ID`     | Google OAuth2 client ID                         |
| `GOOGLE_CLIENT_SECRET` | Google OAuth2 client secret                     |
| `OAUTH_REDIRECT_URL`   | OAuth callback URL                              |
| `EDITOR_EMAILS`        | Comma-separated list of emails with edit access |
| `DATABASE_URL`         | SQLite connection string                        |
| `SEED_DIR`             | Path to seed data directory (default `../seed`) |
