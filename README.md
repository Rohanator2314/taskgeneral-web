# taskgeneral-web

Web interface and REST API server for [taskgeneral-core](https://github.com/Rohanator2314/taskgeneral-core), a task management library built on TaskChampion.

## Overview

- Two components: Rust axum REST API server (server/) and React SPA frontend (frontend/)
- Terminal-inspired, keyboard-driven UI
- Designed for single-user local use
- Production build: single binary serves both API and static frontend

## Architecture

- **server/** — Rust, axum 0.7, wraps taskgeneral-core::TaskManagerWrapper. All core calls run in spawn_blocking. Serves static files from filesystem.
- **frontend/** — React 19, TypeScript, Vite 8, Tailwind CSS v4, TanStack Query. No component libraries — all hand-rolled.

## Prerequisites

- Rust (stable, 2021 edition)
- Node.js 18+
- pnpm

## Getting Started

### Development mode

```bash
# Terminal 1: Start the API server
cd server
cargo run
# Listens on http://localhost:8080

# Terminal 2: Start the frontend dev server
cd frontend
pnpm install
pnpm dev
# Opens http://localhost:5173 (proxies /api → localhost:8080)
```

### Production build

```bash
# Build frontend
cd frontend
pnpm install
pnpm build

# Start server serving static files
cd ../server
TASKGENERAL_STATIC_DIR=../frontend/dist cargo run
# Everything at http://localhost:8080
```

## Configuration (Environment Variables)

| Variable | Default | Description |
|---|---|---|
| TASKGENERAL_PORT | 8080 | Server listen port |
| TASKGENERAL_DATA_DIR | ~/.local/share/taskgeneral/ | Task data storage directory |
| TASKGENERAL_STATIC_DIR | ./frontend/dist | Path to built frontend assets. If absent, server runs in API-only mode. |

## API Reference

**Tasks**

| Method | Endpoint | Description |
|---|---|---|
| GET | /api/tasks | List tasks. Query params: status, project, tag, sort_by |
| POST | /api/tasks | Create task. Body: {"description": "..."} |
| GET | /api/tasks/:uuid | Get single task |
| PUT | /api/tasks/:uuid | Update task fields |
| DELETE | /api/tasks/:uuid | Delete task (soft-delete) |
| POST | /api/tasks/:uuid/complete | Mark task complete |
| POST | /api/tasks/:uuid/uncomplete | Mark task pending |
| POST | /api/tasks/:uuid/start | Start task (set active) |
| POST | /api/tasks/:uuid/stop | Stop task |
| GET | /api/tasks/working-set | Get active working set |

**Sync**

| Method | Endpoint | Description |
|---|---|---|
| POST | /api/sync/configure | Configure sync server. Body: {"server_url", "client_id", "encryption_secret"} |
| POST | /api/sync | Trigger sync |

**System**

| Method | Endpoint | Description |
|---|---|---|
| GET | /api/health | Health check |
| GET | /api/version | Server version |
| DELETE | /api/data | Clear all local data |

Soft-deleted tasks return 404 on GET. Errors return {"error": "message"} with appropriate HTTP status codes (400, 404, 409, 500).

## Keyboard Shortcuts

| Key | Action |
|---|---|
| j / ↓ | Next task |
| k / ↑ | Previous task |
| gg | Jump to first task |
| G | Jump to last task |
| a | Create new task |
| Enter / e | Edit selected task |
| c | Complete task |
| u | Uncomplete task |
| s | Start/stop task (toggle) |
| d | Delete task (with confirmation) |
| / | Focus project filter |
| Escape | Clear selection / cancel |

Keyboard shortcuts are disabled when form inputs are focused or when Ctrl/Cmd is held.

## Testing

```bash
# Server tests (Rust integration tests with real TaskManagerWrapper, no mocks)
cd server
cargo test -- --test-threads=1

# Frontend tests (Vitest + Testing Library + MSW)
cd frontend
pnpm test
```

## Tech Stack

- **Backend**: Rust, axum 0.7, tokio, tower-http, serde, tracing
- **Frontend**: React 19, TypeScript 5.9, Vite 8, Tailwind CSS 4, TanStack Query 5
- **Testing**: Rust integration tests (tempfile), Vitest, Testing Library, MSW 2
