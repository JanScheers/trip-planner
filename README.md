# China 2026 Trip Planner

21-day trip across 7 cities: Beijing → Xi'an → Mount Emei → Chongqing → Zhangjiajie → Guilin → Hong Kong.

## Stack

- **Frontend:** Svelte 5 + TypeScript (Vite)
- **Backend:** Rust + Axum REST API
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
| `FRONTEND_URL`         | Public URL for CORS (default `http://localhost:5173`) |

## Deploying on a VPS (nginx)

After pulling the repo on the VPS:

### 1. Build

```bash
cd frontend && npm ci && npm run build
cd ../backend && cargo build --release
```

### 2. Directory layout on VPS

Clone or pull the repo to e.g. `/opt/trip-planner`. After building:

```
/opt/trip-planner/
├── backend/
│   ├── target/release/backend   # binary
│   ├── static/                 # uploads (created on first run)
│   ├── trip.db                 # SQLite DB (created on first run)
│   └── .env
├── frontend/dist/              # Vite build output
└── seed/                       # seed data
```

### 3. nginx config

Create `/etc/nginx/sites-available/trip-planner`:

```nginx
server {
    listen 80;
    server_name your-domain.com;

    root /opt/trip-planner/frontend/dist;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    location /api {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location /static {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
    }
}
```

Enable and reload:

```bash
sudo ln -s /etc/nginx/sites-available/trip-planner /etc/nginx/sites-enabled/
sudo nginx -t && sudo systemctl reload nginx
```

### 4. Backend `.env` (production)

```env
DATABASE_URL=sqlite:trip.db?mode=rwc
SEED_DIR=/opt/trip-planner/seed
EDITOR_EMAILS=you@example.com

GOOGLE_CLIENT_ID=...
GOOGLE_CLIENT_SECRET=...
OAUTH_REDIRECT_URL=https://your-domain.com/api/auth/callback
FRONTEND_URL=https://your-domain.com
```

In Google Cloud Console, add `https://your-domain.com/api/auth/callback` to Authorized redirect URIs and `https://your-domain.com` to Authorized JavaScript origins.

### 5. systemd service

Create `/etc/systemd/system/trip-planner.service`:

```ini
[Unit]
Description=Trip Planner Backend
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/trip-planner/backend
ExecStart=/opt/trip-planner/backend/target/release/backend
Restart=on-failure
RestartSec=5
EnvironmentFile=/opt/trip-planner/backend/.env

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl daemon-reload
sudo systemctl enable trip-planner
sudo systemctl start trip-planner
```

### 6. HTTPS (optional)

```bash
sudo apt install certbot python3-certbot-nginx
sudo certbot --nginx -d your-domain.com
```
