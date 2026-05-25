# rs-matchmaking

A minimal Rust matchmaking service with a simple HTTP API and a live HTML dashboard.

## Features

- **HTTP API** built with `axum`
- **Matchmaking loop** runs every second and pairs players by MMR
- **Live telemetry dashboard** that polls server state
- **CORS enabled** so the local HTML dashboard can fetch data

## How It Works

- Players join a queue by posting `{ id, mmr }` to `POST /join`.
- Every second, the server sorts the queue by MMR and matches adjacent players.
- A match is formed when the MMR difference is `<= 100`.
- The server keeps the **last 10 matches** in memory for the dashboard.

## Quick Start

```bash
cargo run
```

The server listens on `http://0.0.0.0:3000` and prints the URL on startup.

## API

### POST /join

Add a player to the queue.

```bash
curl -X POST http://localhost:3000/join \
  -H "Content-Type: application/json" \
  -d '{"id":"player-1","mmr":1200}'
```

Response:

```text
Joined queue successfully
```

### GET /state

Fetch queue length and the most recent matches.

```bash
curl http://localhost:3000/state
```

Example response:

```json
{
  "queue_length": 2,
  "recent_matches": [
    {
      "match_id": "d1b0b3fe-4f2a-4b9f-8c08-0dff5b5b0d7b",
      "player_1": {"id":"player-1","mmr":1200},
      "player_2": {"id":"player-2","mmr":1250}
    }
  ]
}
```

## Dashboard

Open [dashboard.html](dashboard.html) in a browser. It polls `http://localhost:3000/state` every 100ms and displays queue length and recent matches.

If you change the server port, also update `RUST_SERVER_URL` in [dashboard.html](dashboard.html).

## Development

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Optional: Python test harness

```bash
python testing.py
```

## Docker

```bash
docker build -t rs-matchmaking .
```

```bash
docker run -p 3000:3000 rs-matchmaking
```

## Project Structure

- [src/main.rs](src/main.rs) — server bootstrap, router, CORS, port
- [src/api.rs](src/api.rs) — HTTP handlers (`/join`, `/state`)
- [src/engine.rs](src/engine.rs) — matchmaking loop and logic
- [src/models.rs](src/models.rs) — data models
- [dashboard.html](dashboard.html) — live telemetry UI

## Notes

- The queue is stored in memory and is not persisted.
- This is a minimal example; add authentication, persistence, and metrics for production use.

## License

No license is currently specified. Add a `LICENSE` file to make usage explicit.
