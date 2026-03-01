# China Trip 
- October 2026
- 21 Days
- 7 Cities (Beijing -> Xi'an -> Chengdu -> Chongqing -> Zhangjiajie -> Guilin -> Hong Kong)

## Stack
- Svelte, Typescript Frontend
- Rust REST API, shared types (ts-rs)
- Sqlite DB
- Google Auth (oauth2 crate, values in .env file)
  - Everybody on the internet can view
  - Some email addresses are whitelisted as editors

## Tables
### Main Table
1 entry per day:
- Date
- City
- Accommodation
- Notes (text column in DB, markdown, initialise empty)
- Emoji (optional)
- Hero Image (optional, uploaded to local static/ directory)

### City Table
- Key
- Name + Chinese Name (initialise empty)
- Notes (text column in DB, markdown, initialise empty)
- Emoji (optional)
- Hero Image (optional, uploaded to local static/ directory)

### Accommodation Table
- Key
- Name
- Main link (to e.g. airbnb) (initialise empty)
- Notes (text column in DB, markdown, initialise empty)
- Emoji (optional)
- Hero Image (optional, uploaded to local static/ directory)


## Frontend
Visual Dark & Gold Style (Chinese theme)

### Home Page
- Countdown timer
- Custom SVG map of China with 7 cities plotted
  - Each city is clickable
  - Cities get filled in as time goes on
  - Styled to match dark & gold theme
- Days of the trip as a segmented progress bar
  - Each segment is clickable

### Basic View
- Overview of all days in a table
- Can add days with + button (copies previous entry)
- Can edit the main fields (City, Accommodation)
- Stretch goal: batch actions (assign city to multiple days at once)

### Day View
- Can edit the entry in more detail
- Can click on the accommodation
- Can click on the city

### City View
- Emoji
- Image of that City
- Overview of days in that city

## Backend
- Import/export trip as a TSV file
- The TSV is tracked in git as the canonical seed data
- On first run (empty DB), the backend initialises from the TSV
- Images uploaded to a local `static/` directory, served by the Rust backend

## Initial TSV
Assign a natural split of 21 days across 7 cities:
- Beijing: 4 days
- Xi'an: 3 days
- Chengdu: 3 days
- Chongqing: 2 days
- Zhangjiajie: 3 days
- Guilin: 3 days
- Hong Kong: 3 days

---

## Build Tasks

### Phase 1: Project Setup
- [ ] 1. Initialise Rust project (Cargo workspace), add dependencies (actix-web, sqlx, serde, ts-rs, oauth2, dotenv)
- [ ] 2. Initialise Svelte + TypeScript frontend (Vite)
- [ ] 3. Create the seed TSV file (`seed.tsv`) with 21 days across 7 cities, track in git

### Phase 2: Backend Core
- [ ] 4. Define Rust structs for Day, City, Accommodation with ts-rs derives
- [ ] 5. Set up SQLite schema (migrations or inline) for the 3 tables
- [ ] 6. Implement TSV import: on first run, seed DB from `seed.tsv`
- [ ] 7. Implement TSV export endpoint (`GET /api/export`)
- [ ] 8. CRUD REST endpoints for Days (`GET /api/days`, `POST`, `PUT`, `DELETE`)
- [ ] 9. CRUD REST endpoints for Cities (`GET /api/cities`, `PUT`)
- [ ] 10. CRUD REST endpoints for Accommodations (`GET /api/accommodations`, `POST`, `PUT`, `DELETE`)
- [ ] 11. Image upload endpoint (`POST /api/upload`) — save to `static/`, return URL
- [ ] 12. Serve `static/` directory for uploaded images

### Phase 3: Auth
- [ ] 13. Google OAuth2 login flow (redirect, callback, session cookie)
- [ ] 14. Middleware: read-only for unauthenticated users, write for whitelisted emails
- [ ] 15. `.env` file with Google client ID/secret and whitelisted editor emails

### Phase 4: Frontend — Basic View
- [ ] 16. Set up API client / fetch helpers with auth headers
- [ ] 17. Basic View: table of all 21 days (date, city, accommodation)
- [ ] 18. Basic View: inline editing of city and accommodation fields
- [ ] 19. Basic View: + button to add a day (copies previous entry)
- [ ] 20. Basic View: delete a day

### Phase 5: Frontend — Day & City Views
- [ ] 21. Day View: full detail page for a single day, edit all fields
- [ ] 22. Day View: markdown editor for notes
- [ ] 23. Day View: image upload for hero image
- [ ] 24. City View: city detail page with emoji, hero image, list of days
- [ ] 25. City View: edit Chinese name, notes, emoji, hero image
- [ ] 26. Accommodation View: detail page with name, link, notes, emoji, hero image

### Phase 6: Frontend — Home Page
- [ ] 27. Countdown timer to October 2026
- [ ] 28. Custom SVG map of China with 7 clickable city markers
- [ ] 29. Segmented progress bar (1 segment per day, clickable)
- [ ] 30. Dark & gold theme styling across all pages

### Phase 7: Polish
- [ ] 31. SVG map: fill in cities as dates pass
- [ ] 32. Responsive layout (mobile-friendly)
- [ ] 33. Stretch: batch assign city to multiple days

