# TaskGeneral Web — Terminal-Inspired Task Manager Frontend

## TL;DR

> **Quick Summary**: Build a web frontend for taskgeneral-core: a Rust axum REST API server (thin HTTP shim over TaskManagerWrapper) + a React/Vite/Tailwind SPA with a terminal-inspired, keyboard-driven UI supporting full task CRUD, filtering/sorting, start/stop, and sync configuration.
> 
> **Deliverables**:
> - Rust axum REST API server (`server/`) with 10 endpoints covering all 16 TaskManagerWrapper methods
> - React + Vite + TypeScript + Tailwind frontend (`frontend/`) with terminal-aesthetic UI
> - Keyboard navigation (vim-style j/k + standard arrows)
> - Dark/light theme toggle with terminal styling
> - Sync configuration UI
> - Server + frontend tests (cargo test + vitest)
> 
> **Estimated Effort**: Large
> **Parallel Execution**: YES — 4 waves
> **Critical Path**: T1 (server scaffold) → T4 (task endpoints) → T5 (list endpoint) → T10 (task list UI) → T13 (keyboard nav) → T16 (integration) → Final

---

## Context

### Original Request
Build a website interface for taskgeneral-core, a platform-agnostic Rust library wrapping TaskChampion 3.0.1 for task management. The recommended architecture is a REST API server (Option A) with a JS/TS frontend.

### Interview Summary
**Key Discussions**:
- **Frontend**: React + Vite + TypeScript + Tailwind CSS — matches project AGENTS.md conventions
- **Backend**: axum REST API — Tokio-native, thin HTTP shim over TaskManagerWrapper
- **UI Direction**: Terminal-inspired, dense, monospace — themeable with light/dark toggle
- **Keyboard**: Both vim-style (j/k/Enter) AND standard (arrows/Enter) navigation
- **State Management**: TanStack Query for all server state; React useState/useContext for local UI state
- **Core Dependency**: Git dependency in Cargo.toml pointing to GitHub repo
- **Deployment**: Local dev tool on localhost — no auth, no CORS needed (Vite proxy)
- **Scope**: CRUD + filtering/sorting + start/stop + complete/uncomplete + keyboard nav + sync config + theme toggle

### Research Findings
- **Tokio Runtime**: Core creates `new_current_thread()` inside TaskManager and calls `runtime.block_on()`. Server MUST use multi-threaded runtime + `spawn_blocking` for all core calls to avoid nested runtime panic.
- **Serde Gap**: Core types `TaskUpdate`, `TaskFilter`, `SortField`, `WorkingSetItem` do NOT derive `Serialize/Deserialize`. Server needs its own DTO types that map to/from core types.
- **API Consolidation**: Core has 3 list methods. Server merges them into a single `GET /api/tasks` with query params.
- **Mutex Serialization**: All core calls go through `Mutex<TaskManager>`. Concurrent API requests are serialized at the core level — acceptable for single-user local tool.

### Metis Review
**Identified Gaps** (addressed):
- **Nested runtime panic risk**: Resolved by using multi-threaded runtime + `spawn_blocking` pattern
- **Missing DTO types**: Plan includes server-side request/response DTOs
- **Missing error mapping**: Plan includes TaskError → HTTP status code mapping
- **Production build strategy**: Single binary serving API + static files via `tower-http::ServeDir`
- **Task edit UX undefined**: Resolved as inline editing in list view (fits terminal aesthetic)
- **Data directory/port not configurable**: Added env var configuration (TASKGENERAL_DATA_DIR, TASKGENERAL_PORT)
- **Sync blocks mutex**: Frontend shows loading state during sync

---

## Work Objectives

### Core Objective
Build a fully functional, terminal-inspired web interface for task management powered by taskgeneral-core, with a Rust REST API backend and React SPA frontend.

### Concrete Deliverables
- `server/` — Rust axum binary crate with REST API endpoints
- `frontend/` — React + Vite + TypeScript + Tailwind SPA
- Working dev setup: `cargo run` (server) + `pnpm dev` (frontend with Vite proxy)
- Production build: `cargo build --release` + `pnpm build`

### Definition of Done
- [ ] `cargo run` in `server/` starts server, `curl http://localhost:8080/api/version` returns version
- [ ] Full task lifecycle via API: create → list → update → start → stop → complete → delete
- [ ] Frontend renders task list with filtering, sorting, and inline editing
- [ ] Keyboard navigation (j/k/arrows) works for task selection and actions
- [ ] Theme toggle persists across sessions (localStorage)
- [ ] Sync configure + sync trigger works from UI
- [ ] `cargo test` passes in server/
- [ ] `pnpm test` passes in frontend/
- [ ] `pnpm build` produces working dist/

### Must Have
- Every axum handler wraps core calls in `tokio::task::spawn_blocking`
- Server uses `#[tokio::main]` (multi-threaded runtime, NOT current_thread)
- Consistent JSON error response format: `{ "error": "message" }`
- Monospace font throughout the UI
- Dense, compact layout (high information density)
- Vite dev proxy (`/api` → `localhost:8080`)
- TanStack Query for all API state with `refetchOnWindowFocus: true`

### Must NOT Have (Guardrails)
- ❌ Authentication, sessions, API keys, JWT, cookies
- ❌ Component libraries (shadcn/ui, Radix, MUI, Chakra)
- ❌ Redux, Zustand, Jotai, or any global state beyond React context
- ❌ SSR, SSG, Next.js, Remix
- ❌ CORS middleware (use Vite proxy instead)
- ❌ Annotations, bulk operations, task dependencies, full-text search
- ❌ Drag-and-drop, command palette (Cmd+K), undo/redo
- ❌ Database/ORM beyond what TaskChampion manages internally
- ❌ Server-side validation beyond what taskgeneral-core enforces
- ❌ Caching layer in the server (core's Mutex serializes access; TanStack Query caches on client)
- ❌ Pagination (core returns all tasks; client handles display)
- ❌ `#[tokio::main(flavor = "current_thread")]` on the server (causes nested runtime panic)
- ❌ Over-commented code, excessive JSDoc, unnecessary abstractions

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO (greenfield — setup included in plan)
- **Automated tests**: YES (tests after implementation)
- **Server**: cargo test with axum::test / tower::ServiceExt, temp directory for TaskManager
- **Frontend**: Vitest + React Testing Library + MSW (Mock Service Worker)
- **E2E**: Playwright via `playwright` skill for keyboard nav and theme verification

### QA Policy
Every task includes agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **API endpoints**: Bash (curl) — send requests, assert status + response fields
- **Frontend UI**: Playwright — navigate, interact, assert DOM, screenshot
- **Keyboard nav**: Playwright — press keys, assert selection state
- **Theme**: Playwright — toggle, assert CSS classes, screenshot
- **Build**: Bash — run build commands, assert exit codes

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — scaffolding + types):
├── Task 1: Rust server project scaffold + axum boilerplate [quick]
├── Task 2: React + Vite + Tailwind project scaffold [quick]
├── Task 3: Server DTO types + error handling [quick]

Wave 2 (After Wave 1 — server endpoints + frontend foundation):
├── Task 4: Task CRUD endpoints (create, get, update, delete) [unspecified-high]
├── Task 5: Task list endpoint (filter + sort) [unspecified-high]
├── Task 6: Task status endpoints (complete, uncomplete, start, stop) [quick]
├── Task 7: Sync + utility endpoints (configure, sync, clear, version, working-set) [quick]
├── Task 8: Frontend theme system + layout shell [visual-engineering]
├── Task 9: Frontend API client + TanStack Query hooks [quick]

Wave 3 (After Wave 2 — frontend features):
├── Task 10: Task list table component [visual-engineering]
├── Task 11: Task create/edit inline form [visual-engineering]
├── Task 12: Filter bar + sort controls [visual-engineering]
├── Task 13: Keyboard navigation system [deep]
├── Task 14: Sync config modal [visual-engineering]

Wave 4 (After Wave 3 — integration + testing):
├── Task 15: Server tests (cargo test) [unspecified-high]
├── Task 16: Frontend tests (vitest) [unspecified-high]
├── Task 17: Production build config (serve static files from Rust) [quick]

Wave FINAL (After ALL tasks — independent review, 4 parallel):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Real manual QA (unspecified-high + playwright)
├── Task F4: Scope fidelity check (deep)

Critical Path: T1 → T3 → T4 → T5 → T9 → T10 → T13 → T16 → F1-F4
Parallel Speedup: ~60% faster than sequential
Max Concurrent: 6 (Waves 2 & 3)
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| T1 | — | T3, T4, T5, T6, T7 | 1 |
| T2 | — | T8, T9 | 1 |
| T3 | T1 | T4, T5, T6, T7 | 1 |
| T4 | T1, T3 | T9, T15 | 2 |
| T5 | T1, T3 | T9, T15 | 2 |
| T6 | T1, T3 | T9, T15 | 2 |
| T7 | T1, T3 | T9, T14, T15 | 2 |
| T8 | T2 | T10, T11, T12, T14 | 2 |
| T9 | T2, T4, T5, T6, T7 | T10, T11, T12, T13, T14, T16 | 2 |
| T10 | T8, T9 | T13, T16 | 3 |
| T11 | T8, T9 | T16 | 3 |
| T12 | T8, T9 | T16 | 3 |
| T13 | T10 | T16 | 3 |
| T14 | T8, T9 | T16 | 3 |
| T15 | T4, T5, T6, T7 | F1-F4 | 4 |
| T16 | T9, T10, T11, T12, T13, T14 | F1-F4 | 4 |
| T17 | T1, T2 | F1-F4 | 4 |

### Agent Dispatch Summary

- **Wave 1 (3 tasks)**: T1 → `quick`, T2 → `quick`, T3 → `quick`
- **Wave 2 (6 tasks)**: T4 → `unspecified-high`, T5 → `unspecified-high`, T6 → `quick`, T7 → `quick`, T8 → `visual-engineering`, T9 → `quick`
- **Wave 3 (5 tasks)**: T10 → `visual-engineering`, T11 → `visual-engineering`, T12 → `visual-engineering`, T13 → `deep`, T14 → `visual-engineering`
- **Wave 4 (3 tasks)**: T15 → `unspecified-high`, T16 → `unspecified-high`, T17 → `quick`
- **FINAL (4 tasks)**: F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

- [x] 1. Rust Server Project Scaffold + Axum Boilerplate

  **What to do**:
  - Create `server/` directory with a Cargo workspace member
  - `server/Cargo.toml`: binary crate depending on `taskgeneral-core` (git = "https://github.com/Rohanator2314/taskgeneral-core.git"), `axum` (0.7+), `tokio` (1, features: full), `serde` (1, features: derive), `serde_json` (1), `tower-http` (0.5+, features: cors, trace), `tracing` (0.1), `tracing-subscriber` (0.3)
  - `server/src/main.rs`: Minimal axum server with `#[tokio::main]` (multi-threaded, NOT current_thread). Create `TaskManagerWrapper` via `create_task_manager()`, wrap in `Arc`, pass as axum `State`. Bind to `0.0.0.0:{TASKGENERAL_PORT}` (default 8080). Read `TASKGENERAL_DATA_DIR` env var (default `~/.local/share/taskgeneral/`). Add a single health check route: `GET /api/health` → `{"status": "ok"}` and `GET /api/version` → `{"version": "<core version>"}`.
  - Create root `Cargo.toml` with `[workspace]` members = `["server"]`
  - Verify the server compiles and starts. First build will be slow (5-15 min) due to taskchampion's bundled SQLite.

  **Must NOT do**:
  - Do NOT add authentication middleware
  - Do NOT add CORS middleware (Vite proxy handles dev, production serves static files)
  - Do NOT use `#[tokio::main(flavor = "current_thread")]`
  - Do NOT add any task-related routes yet

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Boilerplate scaffolding with well-known patterns, no complex logic
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `playwright`: No browser interaction needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3)
  - **Blocks**: Tasks 3, 4, 5, 6, 7, 15, 17
  - **Blocked By**: None (can start immediately)

  **References**:

  **Pattern References**:
  - `taskgeneral-core/src/lib.rs` (in the git repo) — `create_task_manager(data_dir)` function signature and `TaskManagerWrapper` type. This is what the server wraps.
  - `taskgeneral-core/Cargo.toml` (in the git repo) — crate name is `taskgeneral-core`, version 0.1.0

  **API/Type References**:
  - `taskgeneral-core::create_task_manager` → returns `Result<Arc<TaskManagerWrapper>, CoreError>`
  - `taskgeneral-core::version()` → returns `String`

  **External References**:
  - axum docs: https://docs.rs/axum/latest/axum/ — Router, State, handler patterns
  - tokio docs: `#[tokio::main]` defaults to multi-threaded runtime

  **WHY Each Reference Matters**:
  - The core's `create_task_manager` is the entry point — server needs to call it at startup
  - `version()` is a standalone function (not a method) — different call pattern

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Server starts and responds to health check
    Tool: Bash (curl)
    Preconditions: No server running on port 8080
    Steps:
      1. Run `cargo run --manifest-path server/Cargo.toml &` (background, wait 5s for compile+start)
      2. `curl -s -w '\n%{http_code}' http://localhost:8080/api/health`
      3. Assert response body contains `"status":"ok"` and HTTP code is `200`
      4. `curl -s http://localhost:8080/api/version`
      5. Assert response body contains `"version"` key
      6. Kill the server process
    Expected Result: Both endpoints return 200 with valid JSON
    Failure Indicators: Connection refused, non-200 status, missing JSON keys
    Evidence: .sisyphus/evidence/task-1-health-check.txt

  Scenario: Server fails gracefully on port conflict
    Tool: Bash
    Preconditions: Another process listening on port 8080
    Steps:
      1. Start a dummy listener: `nc -l 8080 &`
      2. Run `cargo run --manifest-path server/Cargo.toml 2>&1`
      3. Assert exit code is non-zero
      4. Assert stderr contains "address" or "already in use"
      5. Kill the nc process
    Expected Result: Server exits with clear error message about port conflict
    Failure Indicators: Server hangs, no error message, panic without message
    Evidence: .sisyphus/evidence/task-1-port-conflict.txt
  ```

  **Evidence to Capture:**
  - [ ] task-1-health-check.txt — curl output for health + version endpoints
  - [ ] task-1-port-conflict.txt — error output when port is taken

  **Commit**: YES
  - Message: `feat(server): scaffold axum server with health check`
  - Files: `server/`, `Cargo.toml`
  - Pre-commit: `cargo check --manifest-path server/Cargo.toml`

- [x] 2. React + Vite + Tailwind Project Scaffold

  **What to do**:
  - Run `pnpm create vite@latest frontend -- --template react-ts` to create the React project
  - Install dependencies: `pnpm install` in `frontend/`
  - Install Tailwind CSS v4 (latest): `pnpm add -D tailwindcss @tailwindcss/vite` in `frontend/`
  - Configure Tailwind: Add `@import "tailwindcss"` to `frontend/src/index.css`
  - Configure `@tailwindcss/vite` plugin in `frontend/vite.config.ts`
  - Configure Vite proxy in `frontend/vite.config.ts`: proxy `/api` to `http://localhost:8080`
  - Install TanStack Query: `pnpm add @tanstack/react-query` in `frontend/`
  - Set up `QueryClientProvider` in `frontend/src/main.tsx` with `defaultOptions: { queries: { refetchOnWindowFocus: true } }`
  - Clean out default Vite boilerplate (App.tsx, App.css, etc.) — replace with minimal "TaskGeneral" heading
  - Add root `pnpm-workspace.yaml` with `packages: ["frontend"]`
  - Add root `package.json` with dev scripts
  - Install Vitest: `pnpm add -D vitest @testing-library/react @testing-library/jest-dom jsdom` in `frontend/`
  - Add vitest config to `frontend/vite.config.ts` or `frontend/vitest.config.ts`

  **Must NOT do**:
  - Do NOT install component libraries (shadcn, Radix, MUI, Chakra)
  - Do NOT add Redux, Zustand, or Jotai
  - Do NOT add Next.js, Remix, or SSR tooling
  - Do NOT add MSW yet (added in T16 for tests)
  - Do NOT style beyond a minimal heading — theming is in T8

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Standard Vite scaffolding with well-documented commands
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `frontend-ui-ux`: No styling work yet, just scaffold
    - `playwright`: No browser testing yet

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3)
  - **Blocks**: Tasks 8, 9
  - **Blocked By**: None (can start immediately)

  **References**:

  **External References**:
  - Vite docs: https://vite.dev/guide/ — project creation and config
  - Tailwind v4 docs: https://tailwindcss.com/docs/installation/vite — Vite integration
  - TanStack Query: https://tanstack.com/query/latest/docs/framework/react/overview — QueryClientProvider setup
  - Vitest docs: https://vitest.dev/guide/ — configuration with Vite
  - AGENTS.md: `pnpm create vite@latest <project_name> -- --template react-ts` convention

  **WHY Each Reference Matters**:
  - Tailwind v4 setup is different from v3 — use the `@tailwindcss/vite` plugin, NOT postcss
  - TanStack Query needs `QueryClientProvider` wrapping the app — must be in main.tsx
  - Vite proxy config prevents CORS issues during dev

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Frontend dev server starts and renders
    Tool: Playwright (playwright skill)
    Preconditions: `pnpm install` completed in frontend/
    Steps:
      1. Start dev server: `pnpm dev` in frontend/ (background)
      2. Navigate to http://localhost:5173
      3. Assert page contains text "TaskGeneral" (selector: `body`)
      4. Assert no console errors
      5. Take screenshot
    Expected Result: Page loads with "TaskGeneral" heading, no errors
    Failure Indicators: Blank page, 404, console errors, build errors
    Evidence: .sisyphus/evidence/task-2-frontend-loads.png

  Scenario: Vite proxy forwards /api to backend
    Tool: Bash
    Preconditions: Both server (port 8080) and frontend (port 5173) running
    Steps:
      1. Start server: `cargo run --manifest-path server/Cargo.toml &`
      2. Start frontend: `pnpm dev` in frontend/ (background)
      3. `curl -s http://localhost:5173/api/health`
      4. Assert response contains `"status":"ok"`
    Expected Result: Vite proxies /api requests to the Rust server
    Failure Indicators: 404, CORS error, connection refused
    Evidence: .sisyphus/evidence/task-2-vite-proxy.txt

  Scenario: Frontend build succeeds
    Tool: Bash
    Preconditions: Dependencies installed
    Steps:
      1. Run `pnpm build` in frontend/
      2. Assert exit code is 0
      3. Assert `frontend/dist/index.html` exists
      4. Assert `frontend/dist/assets/` contains .js and .css files
    Expected Result: Production build succeeds, dist/ populated
    Failure Indicators: Build errors, missing dist files
    Evidence: .sisyphus/evidence/task-2-build-output.txt
  ```

  **Evidence to Capture:**
  - [ ] task-2-frontend-loads.png — screenshot of initial page
  - [ ] task-2-vite-proxy.txt — curl output through proxy
  - [ ] task-2-build-output.txt — build command output

  **Commit**: YES
  - Message: `feat(frontend): scaffold React + Vite + Tailwind project`
  - Files: `frontend/`, `pnpm-workspace.yaml`, `package.json`
  - Pre-commit: `pnpm build --filter frontend`

- [x] 3. Server DTO Types + Error Handling

  **What to do**:
  - Create `server/src/types.rs`: Define request/response DTOs that map to/from core types.
    - `CreateTaskRequest { description: String }` (Deserialize)
    - `UpdateTaskRequest { description, project, tags, priority, due, wait, recur: all Option }` (Deserialize) — maps to core's `TaskUpdate`
    - `TaskFilterQuery { status, project, tag, sort_by: all Option<String> }` (Deserialize) — axum Query extractor, maps to core's `TaskFilter` + `SortField`
    - `SyncConfigRequest { server_url, encryption_secret, client_id: String }` (Deserialize)
    - `ErrorResponse { error: String }` (Serialize) — consistent error format
    - `VersionResponse { version: String }` (Serialize)
    - `HealthResponse { status: String }` (Serialize)
    - Re-export core types that already have Serialize: `TaskInfo`, `SyncResult`, `WorkingSetItem` if usable
  - Create `server/src/error.rs`: Define `AppError` type that:
    - Wraps `CoreError` from taskgeneral-core
    - Implements `axum::response::IntoResponse`
    - Maps error variants to HTTP status codes:
      - `TaskNotFound` → 404
      - `InvalidUuid`, `InvalidDescription`, `InvalidPriority`, `InvalidDate`, `InvalidRecurrence` → 400
      - `SyncNotConfigured` → 409 (Conflict)
      - `StorageError`, `SyncError`, `InternalError` → 500
    - Returns JSON: `{ "error": "human-readable message" }`
  - Create `server/src/state.rs`: Define `AppState { manager: Arc<TaskManagerWrapper> }` and implement `Clone`
  - Add conversion functions: `UpdateTaskRequest → TaskUpdate`, `TaskFilterQuery → TaskFilter`, `String → SortField`
  - Wire modules into `server/src/main.rs` via `mod types; mod error; mod state;`

  **Must NOT do**:
  - Do NOT add validation beyond mapping types (core validates)
  - Do NOT add custom serde serializers/deserializers unless absolutely needed
  - Do NOT add generics or trait abstractions — keep it simple flat structs

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Straightforward struct definitions and From/Into impls, no complex logic
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - None relevant

  **Parallelization**:
  - **Can Run In Parallel**: YES (after T1 completes — needs Cargo project to exist)
  - **Parallel Group**: Wave 1 (starts after T1, can run alongside T2)
  - **Blocks**: Tasks 4, 5, 6, 7
  - **Blocked By**: Task 1

  **References**:

  **Pattern References**:
  - `taskgeneral-core/src/models.rs` (in git repo) — `TaskInfo`, `TaskUpdate`, `TaskFilter`, `SortField`, `SyncResult`, `WorkingSetItem` definitions. The server DTOs must match these field-for-field.
  - `taskgeneral-core/src/error.rs` (in git repo) — `CoreError` enum variants. Each variant maps to an HTTP status code.

  **API/Type References**:
  - `TaskUpdate` fields: `description: Option<String>, project: Option<String>, tags: Option<Vec<String>>, priority: Option<String>, due: Option<String>, wait: Option<String>, recur: Option<String>`
  - `TaskFilter` fields: `status: Option<String>, project: Option<String>, tag: Option<String>, sort_by: Option<String>`
  - `SortField` variants: `Urgency, DueDate, Priority, EntryDate, Modified, Description`
  - `CoreError` variants: `TaskNotFound(String), InvalidUuid(String), InvalidDescription(String), InvalidPriority(String), InvalidDate(String), InvalidRecurrence(String), StorageError(String), SyncError(String), SyncNotConfigured, InternalError(String)`

  **External References**:
  - axum IntoResponse: https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html

  **WHY Each Reference Matters**:
  - Core models.rs defines the exact field names/types — server DTOs must match or explicit conversion logic is needed
  - Core error.rs variants determine the HTTP status code mapping — wrong mapping = confusing API errors

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Types compile and conversions work
    Tool: Bash
    Preconditions: Task 1 complete, server project exists
    Steps:
      1. Run `cargo check --manifest-path server/Cargo.toml`
      2. Assert exit code is 0
      3. Assert no warnings related to unused imports or dead code in types.rs, error.rs, state.rs
    Expected Result: All types compile cleanly
    Failure Indicators: Compilation errors, type mismatch, missing imports
    Evidence: .sisyphus/evidence/task-3-cargo-check.txt

  Scenario: Error type produces correct HTTP status codes
    Tool: Bash
    Preconditions: Types defined
    Steps:
      1. Add a temporary test in server/src/error.rs or server/tests/:
         - Create `AppError` from each `CoreError` variant
         - Call `.into_response()` on each
         - Assert correct status codes: TaskNotFound→404, InvalidUuid→400, SyncNotConfigured→409, StorageError→500
      2. Run `cargo test --manifest-path server/Cargo.toml`
      3. Assert all assertions pass
    Expected Result: Each error variant maps to the expected HTTP status code
    Failure Indicators: Wrong status codes, panic on conversion
    Evidence: .sisyphus/evidence/task-3-error-mapping.txt
  ```

  **Evidence to Capture:**
  - [ ] task-3-cargo-check.txt — clean compilation output
  - [ ] task-3-error-mapping.txt — test output for error mapping

  **Commit**: YES
  - Message: `feat(server): add DTO types and error handling`
  - Files: `server/src/types.rs`, `server/src/error.rs`, `server/src/state.rs`, `server/src/main.rs`
  - Pre-commit: `cargo check --manifest-path server/Cargo.toml`

- [x] 4. Task CRUD Endpoints (Create, Get, Update, Delete)

  **What to do**:
  - Create `server/src/handlers.rs` (or `server/src/handlers/tasks.rs` if you prefer a directory)
  - Implement 4 axum handler functions, each following the **mandatory spawn_blocking pattern**:
    ```rust
    async fn handler(State(state): State<AppState>, /* extractors */) -> Result<impl IntoResponse, AppError> {
        let manager = state.manager.clone();
        let result = tokio::task::spawn_blocking(move || {
            manager.method_call(args)
        }).await.unwrap()?;
        Ok(/* response */)
    }
    ```
  - **POST /api/tasks**: Extract `Json<CreateTaskRequest>`, call `manager.create_task(req.description)`, return 201 + `Json<TaskInfo>`
  - **GET /api/tasks/:uuid**: Extract `Path(uuid)`, call `manager.get_task(uuid)`, return 200 + `Json<TaskInfo>` or 404 if None
  - **PATCH /api/tasks/:uuid**: Extract `Path(uuid)` + `Json<UpdateTaskRequest>`, convert to `TaskUpdate`, call `manager.update_task(uuid, update)`, return 200 + `Json<TaskInfo>`
  - **DELETE /api/tasks/:uuid**: Extract `Path(uuid)`, call `manager.delete_task(uuid)`, return 204 No Content
  - Register routes in `main.rs` via `Router::new().route("/api/tasks", post(create_task)).route("/api/tasks/:uuid", get(get_task).patch(update_task).delete(delete_task))`

  **Must NOT do**:
  - Do NOT add validation beyond what the core does (core validates UUID format, description non-empty, etc.)
  - Do NOT add pagination or limiting
  - Do NOT call core methods directly in async context — MUST use `spawn_blocking`
  - Do NOT add caching

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Multiple handler implementations with the critical spawn_blocking pattern — needs care to get right
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - None relevant for Rust backend work

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 6, 7, 8, 9)
  - **Blocks**: Tasks 9, 15
  - **Blocked By**: Tasks 1, 3

  **References**:

  **Pattern References**:
  - `server/src/state.rs` (from T3) — `AppState` struct with `manager: Arc<TaskManagerWrapper>`
  - `server/src/types.rs` (from T3) — `CreateTaskRequest`, `UpdateTaskRequest`, conversions to core types
  - `server/src/error.rs` (from T3) — `AppError` for error handling

  **API/Type References**:
  - `TaskManagerWrapper::create_task(description: String) -> Result<TaskInfo, CoreError>`
  - `TaskManagerWrapper::get_task(uuid: String) -> Result<Option<TaskInfo>, CoreError>`
  - `TaskManagerWrapper::update_task(uuid: String, updates: TaskUpdate) -> Result<TaskInfo, CoreError>`
  - `TaskManagerWrapper::delete_task(uuid: String) -> Result<(), CoreError>`
  - `TaskInfo` — returned from create, get, update. Already has Serialize.

  **External References**:
  - axum extractors: https://docs.rs/axum/latest/axum/extract/index.html — `State`, `Path`, `Json`, `Query`
  - `tokio::task::spawn_blocking`: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html

  **WHY Each Reference Matters**:
  - The `spawn_blocking` pattern is CRITICAL — missing it causes nested runtime panic. Every handler MUST follow this pattern.
  - `get_task` returns `Option<TaskInfo>` — handler must convert `None` to 404

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Full CRUD lifecycle via curl
    Tool: Bash (curl)
    Preconditions: Server running on port 8080 with empty data directory (fresh start)
    Steps:
      1. Create: `curl -s -w '\n%{http_code}' -X POST http://localhost:8080/api/tasks -H 'Content-Type: application/json' -d '{"description":"Buy groceries"}'`
      2. Assert HTTP 201, response contains `"uuid"` and `"description":"Buy groceries"`
      3. Extract UUID from response
      4. Get: `curl -s -w '\n%{http_code}' http://localhost:8080/api/tasks/$UUID`
      5. Assert HTTP 200, response matches created task
      6. Update: `curl -s -w '\n%{http_code}' -X PATCH http://localhost:8080/api/tasks/$UUID -H 'Content-Type: application/json' -d '{"priority":"H","project":"shopping"}'`
      7. Assert HTTP 200, response shows `"priority":"H"` and `"project":"shopping"`
      8. Delete: `curl -s -w '\n%{http_code}' -X DELETE http://localhost:8080/api/tasks/$UUID`
      9. Assert HTTP 204
      10. Get deleted: `curl -s -w '\n%{http_code}' http://localhost:8080/api/tasks/$UUID`
      11. Assert HTTP 404
    Expected Result: Create→Get→Update→Delete→404 cycle completes
    Failure Indicators: Wrong status codes, missing fields in response, panic in server logs
    Evidence: .sisyphus/evidence/task-4-crud-lifecycle.txt

  Scenario: Invalid UUID returns 400
    Tool: Bash (curl)
    Preconditions: Server running
    Steps:
      1. `curl -s -w '\n%{http_code}' http://localhost:8080/api/tasks/not-a-uuid`
      2. Assert HTTP 400
      3. Assert response contains `"error"` key
    Expected Result: 400 with error message
    Failure Indicators: 500, panic, missing error body
    Evidence: .sisyphus/evidence/task-4-invalid-uuid.txt

  Scenario: Create with empty description returns 400
    Tool: Bash (curl)
    Preconditions: Server running
    Steps:
      1. `curl -s -w '\n%{http_code}' -X POST http://localhost:8080/api/tasks -H 'Content-Type: application/json' -d '{"description":""}'`
      2. Assert HTTP 400
      3. Assert response contains `"error"` key
    Expected Result: Core rejects empty description, server returns 400
    Failure Indicators: 201 with empty task, 500, panic
    Evidence: .sisyphus/evidence/task-4-empty-description.txt
  ```

  **Evidence to Capture:**
  - [ ] task-4-crud-lifecycle.txt
  - [ ] task-4-invalid-uuid.txt
  - [ ] task-4-empty-description.txt

  **Commit**: YES (groups with T5, T6, T7)
  - Message: `feat(server): implement all REST API endpoints`
  - Files: `server/src/handlers.rs`, `server/src/main.rs`
  - Pre-commit: `cargo check --manifest-path server/Cargo.toml`

- [x] 5. Task List Endpoint (Filter + Sort)

  **What to do**:
  - Implement a single unified `GET /api/tasks` handler that merges the three core list methods:
    - No query params → calls `list_tasks()`
    - With `status`, `project`, or `tag` params → constructs `TaskFilter`, uses `list_tasks_sorted()` (superset)
    - With `sort_by` param → parses into `SortField` enum
    - Default sort: `SortField::Urgency` when sort_by is omitted
  - Extract query params via `Query<TaskFilterQuery>` from types.rs
  - Parse `sort_by` string to `SortField`: "urgency" → Urgency, "due" → DueDate, "priority" → Priority, "entry" → EntryDate, "modified" → Modified, "description" → Description. Invalid values → 400.
  - Use `spawn_blocking` pattern (mandatory)
  - Response: 200 + `Json<Vec<TaskInfo>>`
  - Register route: `GET /api/tasks` (this is the same path as POST /api/tasks for create, but different method)

  **Must NOT do**:
  - Do NOT add pagination (offset/limit)
  - Do NOT add full-text search
  - Do NOT cache results

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Query param parsing with enum conversion + merging three core methods into one — moderate complexity
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 6, 7, 8, 9)
  - **Blocks**: Tasks 9, 15
  - **Blocked By**: Tasks 1, 3

  **References**:

  **Pattern References**:
  - `server/src/types.rs` (from T3) — `TaskFilterQuery` struct, `SortField` string-to-enum conversion

  **API/Type References**:
  - `TaskManagerWrapper::list_tasks() -> Result<Vec<TaskInfo>, CoreError>`
  - `TaskManagerWrapper::list_tasks_filtered(filter: TaskFilter) -> Result<Vec<TaskInfo>, CoreError>`
  - `TaskManagerWrapper::list_tasks_sorted(filter: TaskFilter, sort_by: SortField) -> Result<Vec<TaskInfo>, CoreError>`
  - `TaskFilter { status: Option<String>, project: Option<String>, tag: Option<String>, sort_by: Option<String> }`
  - `SortField` enum: `Urgency, DueDate, Priority, EntryDate, Modified, Description`

  **External References**:
  - axum Query extractor: https://docs.rs/axum/latest/axum/extract/struct.Query.html

  **WHY Each Reference Matters**:
  - `list_tasks_sorted` is the superset — it takes both filter and sort. Use it for all cases where any param is present.
  - `TaskFilter.sort_by` exists but is ignored by `list_tasks_filtered` — the separate `SortField` param to `list_tasks_sorted` is what matters.

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: List tasks with no filter
    Tool: Bash (curl)
    Preconditions: Server running, 3 tasks created (one with project "work", one with tag "urgent", one plain)
    Steps:
      1. `curl -s http://localhost:8080/api/tasks`
      2. Assert HTTP 200
      3. Assert response is a JSON array with 3 items
      4. Assert each item has `uuid`, `description`, `status`, `urgency` fields
    Expected Result: All pending tasks returned as array
    Failure Indicators: Empty array when tasks exist, 500, missing fields
    Evidence: .sisyphus/evidence/task-5-list-all.txt

  Scenario: Filter by status
    Tool: Bash (curl)
    Preconditions: Server running, at least 1 pending and 1 completed task
    Steps:
      1. `curl -s 'http://localhost:8080/api/tasks?status=completed'`
      2. Assert all returned tasks have `"status":"completed"`
      3. `curl -s 'http://localhost:8080/api/tasks?status=pending'`
      4. Assert all returned tasks have `"status":"pending"`
    Expected Result: Status filter correctly narrows results
    Failure Indicators: Wrong tasks returned, unfiltered results
    Evidence: .sisyphus/evidence/task-5-filter-status.txt

  Scenario: Sort by different fields
    Tool: Bash (curl)
    Preconditions: Server running, 3+ tasks with different properties
    Steps:
      1. `curl -s 'http://localhost:8080/api/tasks?sort_by=description'`
      2. Assert tasks are sorted alphabetically by description
      3. `curl -s 'http://localhost:8080/api/tasks?sort_by=invalid_field'`
      4. Assert HTTP 400 with error message
    Expected Result: Valid sort_by sorts correctly, invalid returns 400
    Failure Indicators: Unsorted results, 500 on invalid sort
    Evidence: .sisyphus/evidence/task-5-sort.txt
  ```

  **Evidence to Capture:**
  - [ ] task-5-list-all.txt
  - [ ] task-5-filter-status.txt
  - [ ] task-5-sort.txt

  **Commit**: YES (groups with T4, T6, T7)
  - Message: (included in T4's commit)
  - Pre-commit: `cargo check --manifest-path server/Cargo.toml`

- [ ] 6. Task Status Endpoints (Complete, Uncomplete, Start, Stop)

  **What to do**:
  - Implement 4 handler functions following the spawn_blocking pattern:
  - **POST /api/tasks/:uuid/complete**: Call `manager.complete_task(uuid)`, return 200 + `Json<TaskInfo>`
  - **POST /api/tasks/:uuid/uncomplete**: Call `manager.uncomplete_task(uuid)`, return 200 + `Json<TaskInfo>`
  - **POST /api/tasks/:uuid/start**: Call `manager.start_task(uuid)`, return 200 + `Json<TaskInfo>`
  - **POST /api/tasks/:uuid/stop**: Call `manager.stop_task(uuid)`, return 200 (stop returns `Result<()>`, not TaskInfo)
  - Register all 4 routes in main.rs

  **Must NOT do**:
  - Do NOT add bulk operations (complete/start multiple tasks at once)
  - Do NOT call core methods without spawn_blocking

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Four simple handlers following the same pattern as T4 — straightforward
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 5, 7, 8, 9)
  - **Blocks**: Tasks 9, 15
  - **Blocked By**: Tasks 1, 3

  **References**:

  **API/Type References**:
  - `TaskManagerWrapper::complete_task(uuid: String) -> Result<TaskInfo, CoreError>`
  - `TaskManagerWrapper::uncomplete_task(uuid: String) -> Result<TaskInfo, CoreError>`
  - `TaskManagerWrapper::start_task(uuid: String) -> Result<TaskInfo, CoreError>`
  - `TaskManagerWrapper::stop_task(uuid: String) -> Result<(), CoreError>` — NOTE: returns `()`, not `TaskInfo`

  **Pattern References**:
  - `server/src/handlers.rs` (from T4) — follow exact same handler pattern

  **WHY Each Reference Matters**:
  - `stop_task` returns `()` not `TaskInfo` — handler response must differ (just return 200 with no body, or re-fetch the task)

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Start → Stop → Complete → Uncomplete lifecycle
    Tool: Bash (curl)
    Preconditions: Server running, one pending task created
    Steps:
      1. Create task, extract UUID
      2. Start: `curl -s -w '\n%{http_code}' -X POST http://localhost:8080/api/tasks/$UUID/start`
      3. Assert HTTP 200, response shows `"is_active":true`
      4. Stop: `curl -s -w '\n%{http_code}' -X POST http://localhost:8080/api/tasks/$UUID/stop`
      5. Assert HTTP 200
      6. Complete: `curl -s -w '\n%{http_code}' -X POST http://localhost:8080/api/tasks/$UUID/complete`
      7. Assert HTTP 200, response shows `"status":"completed"`
      8. Uncomplete: `curl -s -w '\n%{http_code}' -X POST http://localhost:8080/api/tasks/$UUID/uncomplete`
      9. Assert HTTP 200, response shows `"status":"pending"`
    Expected Result: Full status lifecycle completes
    Failure Indicators: Wrong status values, 500, panic
    Evidence: .sisyphus/evidence/task-6-status-lifecycle.txt

  Scenario: Status action on non-existent task returns 404
    Tool: Bash (curl)
    Preconditions: Server running
    Steps:
      1. `curl -s -w '\n%{http_code}' -X POST http://localhost:8080/api/tasks/00000000-0000-0000-0000-000000000000/complete`
      2. Assert HTTP 404
    Expected Result: 404 with error message
    Failure Indicators: 500, panic
    Evidence: .sisyphus/evidence/task-6-status-not-found.txt
  ```

  **Evidence to Capture:**
  - [ ] task-6-status-lifecycle.txt
  - [ ] task-6-status-not-found.txt

  **Commit**: YES (groups with T4, T5, T7)
  - Message: (included in T4's commit)
  - Pre-commit: `cargo check --manifest-path server/Cargo.toml`

- [ ] 7. Sync + Utility Endpoints (Configure, Sync, Clear, Version, Working Set)

  **What to do**:
  - Implement 5 handler functions following the spawn_blocking pattern:
  - **POST /api/sync/configure**: Extract `Json<SyncConfigRequest>`, call `manager.configure_sync(server_url, encryption_secret, client_id)`, return 200 + `{"status": "configured"}`
  - **POST /api/sync**: Call `manager.sync()`, return 200 + `Json<SyncResult>`. NOTE: sync may take several seconds — frontend should handle timeout.
  - **POST /api/data/clear**: Call `manager.clear_local_data()`, return 200 + `{"status": "cleared"}`. This is destructive — handler should log a warning.
  - **GET /api/version**: Call `taskgeneral_core::version()`, return 200 + `Json<VersionResponse>`. NOTE: This is a free function, not a method on the wrapper.
  - **GET /api/working-set**: Call `manager.get_working_set()`, return 200 + `Json<Vec<WorkingSetItem>>`
  - Register all routes in main.rs

  **Must NOT do**:
  - Do NOT add auth to protect destructive endpoints (clear)
  - Do NOT add a confirmation mechanism in the API (that's frontend's job)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Same spawn_blocking handler pattern, simple endpoints
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 5, 6, 8, 9)
  - **Blocks**: Tasks 9, 14, 15
  - **Blocked By**: Tasks 1, 3

  **References**:

  **API/Type References**:
  - `TaskManagerWrapper::configure_sync(server_url: String, encryption_secret: String, client_id: String) -> Result<(), CoreError>`
  - `TaskManagerWrapper::sync() -> Result<SyncResult, CoreError>`
  - `TaskManagerWrapper::clear_local_data() -> Result<(), CoreError>`
  - `TaskManagerWrapper::get_working_set() -> Result<Vec<WorkingSetItem>, CoreError>`
  - `taskgeneral_core::version() -> String` — NOTE: standalone function, NOT on TaskManagerWrapper
  - `SyncResult { success: bool, message: String }`
  - `WorkingSetItem { id: u64, task: TaskInfo }`

  **Pattern References**:
  - `server/src/types.rs` (from T3) — `SyncConfigRequest`, `VersionResponse`
  - `server/src/handlers.rs` (from T4) — handler pattern

  **WHY Each Reference Matters**:
  - `version()` is a free function — don't try to call it on the manager
  - `sync()` may be slow (network I/O) — the spawn_blocking is especially important here
  - `clear_local_data()` is destructive — important to log it

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Version endpoint returns version string
    Tool: Bash (curl)
    Preconditions: Server running
    Steps:
      1. `curl -s http://localhost:8080/api/version`
      2. Assert response contains `"version"` key with non-empty string value
    Expected Result: 200 with version
    Failure Indicators: Missing version, empty string
    Evidence: .sisyphus/evidence/task-7-version.txt

  Scenario: Working set returns array
    Tool: Bash (curl)
    Preconditions: Server running, at least 1 pending task
    Steps:
      1. Create a task
      2. `curl -s http://localhost:8080/api/working-set`
      3. Assert HTTP 200
      4. Assert response is array, each item has `"id"` (number) and `"task"` (TaskInfo object)
    Expected Result: Working set returned with sequential IDs
    Failure Indicators: Empty array when tasks exist, missing id field
    Evidence: .sisyphus/evidence/task-7-working-set.txt

  Scenario: Sync without configuration returns error
    Tool: Bash (curl)
    Preconditions: Server running, sync NOT configured
    Steps:
      1. `curl -s -w '\n%{http_code}' -X POST http://localhost:8080/api/sync`
      2. Assert HTTP 409 (Conflict, via SyncNotConfigured error mapping)
      3. Assert response contains `"error"` key
    Expected Result: 409 with descriptive error about sync not being configured
    Failure Indicators: 500, panic, misleading error
    Evidence: .sisyphus/evidence/task-7-sync-not-configured.txt
  ```

  **Evidence to Capture:**
  - [ ] task-7-version.txt
  - [ ] task-7-working-set.txt
  - [ ] task-7-sync-not-configured.txt

  **Commit**: YES (groups with T4, T5, T6)
  - Message: (included in T4's commit)
  - Pre-commit: `cargo test --manifest-path server/Cargo.toml`

- [x] 8. Frontend Theme System + Layout Shell

  **What to do**:
  - Create a theme system with dark (default) and light modes:
    - `frontend/src/theme/ThemeContext.tsx`: React context providing `{ theme: 'dark' | 'light', toggleTheme: () => void }`
    - On mount, read from `localStorage.getItem('tg-theme')`. Default to `'dark'`.
    - On toggle, update localStorage and set `document.documentElement.classList` to `'dark'` or `'light'`
  - Configure Tailwind for terminal aesthetic:
    - Set monospace font as default: JetBrains Mono (Google Fonts CDN) with `monospace` fallback
    - Define color palette via CSS custom properties (Tailwind v4 `@theme` directive):
      - Dark: bg `#0d1117` (GitHub dark-like), text `#c9d1d9`, accent `#58a6ff`, borders `#30363d`
      - Light: bg `#f6f8fa`, text `#24292f`, accent `#0969da`, borders `#d0d7de`
    - Dense spacing: default padding/margin scale is compact
  - Create layout shell component `frontend/src/components/Layout.tsx`:
    - Header bar: app name "TaskGeneral" (left), theme toggle button (right), version display
    - Main content area (where task list will go)
    - Footer/status bar: connection status indicator, last sync time (hardcoded placeholders for now)
    - Terminal-inspired border styling (single-line borders, box characters optional)
  - Create theme toggle button component with sun/moon icon (inline SVG or text `☀`/`🌙`)
  - Wire ThemeProvider into `main.tsx`
  - Add `<link>` for JetBrains Mono font in `index.html`

  **Must NOT do**:
  - Do NOT install a component library for the toggle
  - Do NOT install an icon library — use inline SVG or unicode characters
  - Do NOT add any task-related UI (that's T10-T14)
  - Do NOT add animations or transitions

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: UI/visual work — theming, color palette, typography, layout
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: Terminal-aesthetic design decisions, color palette, typography
  - **Skills Evaluated but Omitted**:
    - `playwright`: No QA verification at this stage — just visual foundation

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4-7, 9)
  - **Blocks**: Tasks 10, 11, 12, 14
  - **Blocked By**: Task 2

  **References**:

  **Pattern References**:
  - `frontend/src/main.tsx` (from T2) — where to add ThemeProvider wrapper
  - `frontend/src/index.css` (from T2) — where Tailwind is imported, add theme variables

  **External References**:
  - Tailwind v4 theming: https://tailwindcss.com/docs/theme — `@theme` directive for custom tokens
  - JetBrains Mono: https://fonts.google.com/specimen/JetBrains+Mono — CDN link
  - Tailwind dark mode: https://tailwindcss.com/docs/dark-mode — class-based dark mode strategy

  **WHY Each Reference Matters**:
  - Tailwind v4 uses `@theme` for custom values — different from v3's `tailwind.config.js`
  - Class-based dark mode (`class` strategy) is needed for manual toggle — not system-preference-based

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Theme toggle switches between dark and light
    Tool: Playwright (playwright skill)
    Preconditions: Frontend dev server running
    Steps:
      1. Navigate to http://localhost:5173
      2. Assert `document.documentElement.classList` contains `dark` (default)
      3. Assert body background is dark (#0d1117 or similar)
      4. Take screenshot: dark-theme.png
      5. Click theme toggle button (selector: `[data-testid="theme-toggle"]` or button in header)
      6. Assert `document.documentElement.classList` contains `light`
      7. Assert body background is light (#f6f8fa or similar)
      8. Take screenshot: light-theme.png
    Expected Result: Theme toggles visually, classes update
    Failure Indicators: No visual change, class not updating, flash of wrong theme
    Evidence: .sisyphus/evidence/task-8-dark-theme.png, .sisyphus/evidence/task-8-light-theme.png

  Scenario: Theme persists across page reload
    Tool: Playwright (playwright skill)
    Preconditions: Frontend running, default dark theme
    Steps:
      1. Navigate to http://localhost:5173
      2. Click theme toggle to switch to light
      3. Reload the page
      4. Assert `document.documentElement.classList` contains `light` (persisted)
    Expected Result: Theme persists via localStorage
    Failure Indicators: Reverts to dark after reload
    Evidence: .sisyphus/evidence/task-8-theme-persistence.png

  Scenario: Layout shell renders correctly
    Tool: Playwright (playwright skill)
    Preconditions: Frontend running
    Steps:
      1. Navigate to http://localhost:5173
      2. Assert header exists with text "TaskGeneral"
      3. Assert theme toggle button is visible
      4. Assert monospace font is applied (computed font-family includes "JetBrains Mono" or "monospace")
      5. Take full-page screenshot
    Expected Result: Terminal-aesthetic layout with header, content area, status bar
    Failure Indicators: Missing elements, wrong font, broken layout
    Evidence: .sisyphus/evidence/task-8-layout-shell.png
  ```

  **Evidence to Capture:**
  - [ ] task-8-dark-theme.png
  - [ ] task-8-light-theme.png
  - [ ] task-8-theme-persistence.png
  - [ ] task-8-layout-shell.png

  **Commit**: YES
  - Message: `feat(frontend): add theme system and layout shell`
  - Files: `frontend/src/theme/`, `frontend/src/components/Layout.tsx`, `frontend/src/index.css`, `frontend/index.html`
  - Pre-commit: `pnpm build --filter frontend`

- [ ] 9. Frontend API Client + TanStack Query Hooks

  **What to do**:
  - Create `frontend/src/api/client.ts`: Thin fetch wrapper for all API calls
    - Base URL: `/api` (Vite proxy handles routing to backend)
    - Helper: `async function apiFetch<T>(path: string, options?: RequestInit): Promise<T>` — handles JSON parsing, error extraction
    - Error handling: If response is not OK, parse `{ error: string }` body and throw typed `ApiError`
    - Functions matching every server endpoint:
      - `createTask(description: string): Promise<TaskInfo>`
      - `getTask(uuid: string): Promise<TaskInfo>`
      - `listTasks(filter?: TaskFilterParams): Promise<TaskInfo[]>`
      - `updateTask(uuid: string, updates: TaskUpdateParams): Promise<TaskInfo>`
      - `deleteTask(uuid: string): Promise<void>`
      - `completeTask(uuid: string): Promise<TaskInfo>`
      - `uncompleteTask(uuid: string): Promise<TaskInfo>`
      - `startTask(uuid: string): Promise<TaskInfo>`
      - `stopTask(uuid: string): Promise<void>`
      - `configureSyncServer(config: SyncConfig): Promise<void>`
      - `syncNow(): Promise<SyncResult>`
      - `clearData(): Promise<void>`
      - `getVersion(): Promise<string>`
      - `getWorkingSet(): Promise<WorkingSetItem[]>`
  - Create `frontend/src/api/types.ts`: TypeScript types mirroring the server DTOs
    - `TaskInfo`, `TaskUpdateParams`, `TaskFilterParams`, `SyncConfig`, `SyncResult`, `WorkingSetItem`
    - These mirror the JSON shapes from the API
  - Create `frontend/src/api/hooks.ts`: TanStack Query hooks
    - `useTaskList(filter?: TaskFilterParams)` — `useQuery` wrapping `listTasks`
    - `useTask(uuid: string)` — `useQuery` wrapping `getTask`
    - `useWorkingSet()` — `useQuery` wrapping `getWorkingSet`
    - `useCreateTask()` — `useMutation` wrapping `createTask`, invalidates `['tasks']`
    - `useUpdateTask()` — `useMutation` wrapping `updateTask`, invalidates `['tasks']`
    - `useDeleteTask()` — `useMutation` wrapping `deleteTask`, invalidates `['tasks']`
    - `useCompleteTask()` — `useMutation` wrapping `completeTask`, invalidates `['tasks']`
    - `useUncompleteTask()` — `useMutation` wrapping `uncompleteTask`, invalidates `['tasks']`
    - `useStartTask()` — `useMutation` wrapping `startTask`, invalidates `['tasks']`
    - `useStopTask()` — `useMutation` wrapping `stopTask`, invalidates `['tasks']`
    - `useSyncConfig()` — `useMutation` wrapping `configureSyncServer`
    - `useSync()` — `useMutation` wrapping `syncNow`
    - `useClearData()` — `useMutation` wrapping `clearData`, invalidates all
    - `useVersion()` — `useQuery` wrapping `getVersion`, `staleTime: Infinity`
  - Query keys: `['tasks', filter]`, `['task', uuid]`, `['working-set']`, `['version']`
  - All mutations invalidate `['tasks']` query to refetch after changes

  **Must NOT do**:
  - Do NOT add optimistic updates (keep it simple — wait for server confirmation)
  - Do NOT add WebSocket or polling (rely on `refetchOnWindowFocus`)
  - Do NOT install axios — use native `fetch`
  - Do NOT add caching configuration beyond TanStack Query defaults

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Well-documented TanStack Query patterns, no complex logic — just wrapping fetch calls
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `frontend-ui-ux`: No UI work, just data layer

  **Parallelization**:
  - **Can Run In Parallel**: NO (needs all server endpoints defined to know exact API surface)
  - **Parallel Group**: Wave 2 (starts after T4-T7 define the API surface)
  - **Blocks**: Tasks 10, 11, 12, 13, 14, 16
  - **Blocked By**: Tasks 2, 4, 5, 6, 7

  **References**:

  **Pattern References**:
  - `server/src/handlers.rs` (from T4-T7) — exact API routes and request/response shapes
  - `server/src/types.rs` (from T3) — DTO shapes that TypeScript types must mirror

  **API/Type References**:
  - All server endpoints from T4-T7 (complete list above)
  - `TaskInfo` shape: `{ uuid, description, status, project?, tags, priority?, entry?, modified?, due?, wait?, start?, recur?, urgency, is_active, is_waiting }`

  **External References**:
  - TanStack Query: https://tanstack.com/query/latest/docs/framework/react/reference/useQuery
  - TanStack Mutations: https://tanstack.com/query/latest/docs/framework/react/guides/mutations

  **WHY Each Reference Matters**:
  - Server endpoint shapes determine TypeScript types — they must match exactly
  - TanStack Query key strategy determines cache invalidation — `['tasks', filter]` ensures filtered/sorted views share the cache correctly

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: API client creates and lists tasks through hooks
    Tool: Playwright (playwright skill)
    Preconditions: Server running on 8080, frontend running on 5173
    Steps:
      1. Navigate to http://localhost:5173
      2. Open browser console
      3. Verify no API errors in console
      4. (If task list is wired in T10 — skip this scenario and defer to T10 QA)
      5. Alternatively: Run `pnpm build` in frontend to verify types compile
    Expected Result: Types compile, build succeeds
    Failure Indicators: TypeScript errors, missing types
    Evidence: .sisyphus/evidence/task-9-types-compile.txt

  Scenario: API client handles server errors
    Tool: Bash
    Preconditions: Frontend built
    Steps:
      1. Run `pnpm build --filter frontend`
      2. Assert exit code 0 (all types resolve)
      3. Verify api/client.ts exports all expected functions (grep for function names)
      4. Verify api/hooks.ts exports all expected hooks (grep for hook names)
    Expected Result: Complete API surface defined and compiles
    Failure Indicators: Missing exports, type errors
    Evidence: .sisyphus/evidence/task-9-api-surface.txt
  ```

  **Evidence to Capture:**
  - [ ] task-9-types-compile.txt
  - [ ] task-9-api-surface.txt

  **Commit**: YES
  - Message: `feat(frontend): add API client and TanStack Query hooks`
  - Files: `frontend/src/api/`
  - Pre-commit: `pnpm build --filter frontend`

- [ ] 10. Task List Table Component

  **What to do**:
  - Create `frontend/src/components/TaskList.tsx`: The main task list component
    - Uses `useTaskList()` hook to fetch tasks
    - Renders a table with columns: `#` (working set ID), Description, Status, Project, Tags, Priority, Due, Age, Urgency
    - Each row represents a `TaskInfo` object
    - Rows are selectable — one row can be "selected" at a time (highlighted with accent color border/background)
    - Selected row state managed via `useState<number>` (index) — exposed via ref/callback for keyboard nav (T13)
    - Empty state: "No tasks found. Press `a` to add a new task." when list is empty
    - Empty filter state: "No tasks match the current filter." when filter returns nothing
    - Loading state: skeleton/shimmer or "Loading tasks..." text
    - Error state: "Failed to load tasks. Retrying..." with retry button
    - Age column: calculated from `entry` field (human-readable: "2d", "3w", "1mo")
    - Tags displayed as space-separated inline (e.g., `+urgent +home`)
    - Priority displayed as single letter (H/M/L) or empty
    - Dense row height — no wasted vertical space
    - Urgency displayed to 1 decimal place
    - Active tasks (is_active=true) have a visual indicator (e.g., `▶` prefix or highlighted row)
    - Status column shows: `P` (pending), `C` (completed), `D` (deleted)
  - Wire into Layout.tsx as the main content area
  - Style with terminal aesthetic: monospace, borders between columns, compact cells

  **Must NOT do**:
  - Do NOT add virtual scrolling (MVP — render all rows)
  - Do NOT add column resizing or custom columns
  - Do NOT add drag-and-drop reordering
  - Do NOT add inline editing here (that's T11)
  - Do NOT add pagination

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Core visual component — table layout, terminal aesthetic, responsive design
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: Dense table design, terminal aesthetic, information hierarchy
  - **Skills Evaluated but Omitted**:
    - `playwright`: QA is separate concern

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 11, 12, 13, 14)
  - **Blocks**: Tasks 13, 16
  - **Blocked By**: Tasks 8, 9

  **References**:

  **Pattern References**:
  - `frontend/src/api/hooks.ts` (from T9) — `useTaskList()` hook for data fetching
  - `frontend/src/api/types.ts` (from T9) — `TaskInfo` type for row rendering
  - `frontend/src/components/Layout.tsx` (from T8) — where to mount the task list
  - `frontend/src/theme/ThemeContext.tsx` (from T8) — theme-aware styling

  **External References**:
  - Taskwarrior CLI output for reference: columns are `ID Desc Status Project Tags Pri Due Age Urg`

  **WHY Each Reference Matters**:
  - `useTaskList()` returns `{ data, isLoading, isError, error }` — component needs to handle all states
  - Taskwarrior's CLI output is the aesthetic reference — dense, aligned columns, minimal decoration

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Task list renders with data
    Tool: Playwright (playwright skill)
    Preconditions: Server running with 3+ tasks (varying projects, tags, priorities)
    Steps:
      1. Navigate to http://localhost:5173
      2. Assert a table/grid element exists (selector: `[data-testid="task-list"]` or `table`)
      3. Assert at least 3 rows visible
      4. Assert columns are present: verify header text includes "Description", "Status", "Urgency"
      5. Assert monospace font on table cells
      6. Take screenshot
    Expected Result: Dense terminal-styled table with task data
    Failure Indicators: No table, missing columns, wrong data, non-monospace font
    Evidence: .sisyphus/evidence/task-10-task-list.png

  Scenario: Empty state shows appropriate message
    Tool: Playwright (playwright skill)
    Preconditions: Server running with empty task database (fresh data dir)
    Steps:
      1. Navigate to http://localhost:5173
      2. Assert page contains text matching "No tasks" or "no tasks"
      3. Assert the message mentions pressing 'a' to add
    Expected Result: Helpful empty state message
    Failure Indicators: Blank content area, error message, spinner that never resolves
    Evidence: .sisyphus/evidence/task-10-empty-state.png

  Scenario: Row selection highlighting
    Tool: Playwright (playwright skill)
    Preconditions: Server running with 3+ tasks
    Steps:
      1. Navigate to http://localhost:5173
      2. Click on the second task row
      3. Assert the clicked row has distinct styling (different background/border color)
      4. Assert only one row is highlighted at a time
    Expected Result: Clicked row is visually selected
    Failure Indicators: No highlight, multiple highlights, wrong row
    Evidence: .sisyphus/evidence/task-10-row-selection.png
  ```

  **Evidence to Capture:**
  - [ ] task-10-task-list.png
  - [ ] task-10-empty-state.png
  - [ ] task-10-row-selection.png

  **Commit**: YES (groups with T11, T12)
  - Message: `feat(frontend): add task list, inline editing, and filter controls`
  - Files: `frontend/src/components/TaskList.tsx`
  - Pre-commit: `pnpm build --filter frontend`

- [ ] 11. Task Create/Edit Inline Form

  **What to do**:
  - Create `frontend/src/components/TaskForm.tsx`: Inline form for creating and editing tasks
  - **Create mode** (triggered by pressing `a` key or "Add" button):
    - Renders at the top of the task list (above the table)
    - Single text input for description
    - Press `Enter` to submit, `Escape` to cancel
    - On submit: calls `useCreateTask().mutate()` → clears form → refocuses list
  - **Edit mode** (triggered by pressing `Enter` or `e` on a selected task):
    - Replaces the selected row with editable fields inline
    - Editable fields: description (text input), project (text input), priority (dropdown: H/M/L/none), due (date input), tags (text input, space-separated)
    - Pre-populated with current task values
    - Press `Enter` or `Tab` out to save, `Escape` to cancel
    - On save: calls `useUpdateTask().mutate({ uuid, updates })` → exits edit mode → refocuses list
  - **Action buttons** on selected row (or keyboard shortcuts — implemented in T13):
    - Complete/Uncomplete: calls `useCompleteTask()` / `useUncompleteTask()`
    - Start/Stop: calls `useStartTask()` / `useStopTask()`
    - Delete: calls `useDeleteTask()` with a confirmation prompt (simple `window.confirm`)
  - All mutations show inline feedback: brief "Saved" / "Deleted" flash, or error message
  - Form inputs inherit terminal styling (monospace, dark/light theme, border style matching table)

  **Must NOT do**:
  - Do NOT create a separate page or modal for task editing (inline only)
  - Do NOT add fields not supported by `TaskUpdate` (no annotations, dependencies, etc.)
  - Do NOT add rich text editing
  - Do NOT add autocomplete for projects or tags (MVP)
  - Do NOT wire keyboard shortcuts here (that's T13 — this task just exposes the trigger functions)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Form UI with inline editing pattern — needs visual precision
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: Inline editing UX patterns, form layout in terminal aesthetic
  - **Skills Evaluated but Omitted**:
    - `playwright`: QA is separate concern

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 10, 12, 13, 14)
  - **Blocks**: Task 16
  - **Blocked By**: Tasks 8, 9

  **References**:

  **Pattern References**:
  - `frontend/src/api/hooks.ts` (from T9) — mutation hooks: `useCreateTask`, `useUpdateTask`, `useCompleteTask`, etc.
  - `frontend/src/api/types.ts` (from T9) — `TaskUpdateParams` shape for edit form
  - `frontend/src/components/TaskList.tsx` (from T10) — where inline form integrates

  **API/Type References**:
  - `TaskUpdateParams`: `{ description?, project?, tags?, priority?, due?, wait?, recur? }` — all optional
  - `useCreateTask().mutate(description)` → `Promise<TaskInfo>`
  - `useUpdateTask().mutate({ uuid, ...updates })` → `Promise<TaskInfo>`

  **WHY Each Reference Matters**:
  - `TaskUpdateParams` fields determine which form inputs to show — only those 7 fields
  - Mutation hooks handle cache invalidation automatically — no manual refetch needed

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Create a new task via inline form
    Tool: Playwright (playwright skill)
    Preconditions: Server running, frontend running
    Steps:
      1. Navigate to http://localhost:5173
      2. Click "Add" button or trigger create mode
      3. Assert input field appears at top of list
      4. Type "Buy milk" into description input
      5. Press Enter
      6. Assert new task "Buy milk" appears in the task list
      7. Assert input field is cleared/hidden
    Expected Result: Task created and visible in list
    Failure Indicators: Task not created, form doesn't clear, API error
    Evidence: .sisyphus/evidence/task-11-create-task.png

  Scenario: Edit an existing task inline
    Tool: Playwright (playwright skill)
    Preconditions: Server running with at least 1 task
    Steps:
      1. Navigate to http://localhost:5173
      2. Click on a task row to select it
      3. Trigger edit mode (double-click or press Enter/e — whichever is wired)
      4. Assert row transforms into editable inputs
      5. Change description to "Updated task"
      6. Change priority to "H"
      7. Save (Enter or blur)
      8. Assert row shows updated values: "Updated task" with priority "H"
    Expected Result: Task updated inline, changes reflected
    Failure Indicators: Edit mode not triggered, changes not saved, API error
    Evidence: .sisyphus/evidence/task-11-edit-task.png

  Scenario: Delete task with confirmation
    Tool: Playwright (playwright skill)
    Preconditions: Server running with at least 1 task
    Steps:
      1. Navigate to http://localhost:5173
      2. Select a task, note its description
      3. Trigger delete action
      4. Assert confirmation dialog appears (window.confirm or custom)
      5. Confirm deletion
      6. Assert task no longer appears in list
    Expected Result: Task deleted after confirmation
    Failure Indicators: No confirmation, task still visible, API error
    Evidence: .sisyphus/evidence/task-11-delete-task.png
  ```

  **Evidence to Capture:**
  - [ ] task-11-create-task.png
  - [ ] task-11-edit-task.png
  - [ ] task-11-delete-task.png

  **Commit**: YES (groups with T10, T12)
  - Message: (included in T10's commit)
  - Pre-commit: `pnpm build --filter frontend`

- [ ] 12. Filter Bar + Sort Controls

  **What to do**:
  - Create `frontend/src/components/FilterBar.tsx`: Filter and sort controls above the task list
  - **Filter controls**:
    - Status dropdown: All (default) / Pending / Completed / Waiting
    - Project text input: filter by project name (applies on Enter or blur)
    - Tag text input: filter by tag name (applies on Enter or blur)
  - **Sort control**:
    - Dropdown or clickable column headers: Urgency (default) / Due Date / Priority / Entry Date / Modified / Description
  - State management:
    - Filter/sort state stored in URL query params via `useSearchParams()`:
      - `?status=pending&project=work&tag=urgent&sort_by=due`
    - On change, update URL params → `useTaskList(filter)` hook re-fetches with new params
    - This makes filter state sharable and persistent across page reloads
  - Visual styling:
    - Terminal aesthetic — monospace labels, compact controls
    - Active filters visually indicated (e.g., different color/underline)
    - "Clear filters" button/link when any filter is active
  - Wire into Layout.tsx between header and task list

  **Must NOT do**:
  - Do NOT add full-text search
  - Do NOT add filter presets or saved filters
  - Do NOT add complex filter logic (AND/OR combinations)
  - Do NOT add date range filters

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: UI component with form controls and URL state management
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: Filter UX patterns, compact control layout
  - **Skills Evaluated but Omitted**:
    - `playwright`: QA is separate concern

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 10, 11, 13, 14)
  - **Blocks**: Task 16
  - **Blocked By**: Tasks 8, 9

  **References**:

  **Pattern References**:
  - `frontend/src/api/hooks.ts` (from T9) — `useTaskList(filter)` accepts `TaskFilterParams`
  - `frontend/src/api/types.ts` (from T9) — `TaskFilterParams { status?, project?, tag?, sort_by? }`

  **External References**:
  - React Router `useSearchParams`: https://reactrouter.com/en/main/hooks/use-search-params — URL-based state
  - NOTE: If React Router is not installed, use `window.location.search` + `URLSearchParams` manually

  **WHY Each Reference Matters**:
  - `TaskFilterParams` fields determine which filter controls exist — exactly 4: status, project, tag, sort_by
  - URL params enable bookmark/share and survive page refresh

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Filter by status
    Tool: Playwright (playwright skill)
    Preconditions: Server running with mix of pending and completed tasks
    Steps:
      1. Navigate to http://localhost:5173
      2. Assert all tasks visible (default: all or pending)
      3. Select "Completed" from status dropdown
      4. Assert URL contains `?status=completed`
      5. Assert only completed tasks shown
      6. Select "All" to clear filter
      7. Assert all tasks visible again
    Expected Result: Status filter narrows task list
    Failure Indicators: Filter not applied, wrong tasks shown, URL not updated
    Evidence: .sisyphus/evidence/task-12-filter-status.png

  Scenario: Sort by due date
    Tool: Playwright (playwright skill)
    Preconditions: Server running with tasks having different due dates
    Steps:
      1. Navigate to http://localhost:5173
      2. Change sort to "Due Date"
      3. Assert URL contains `sort_by=due`
      4. Assert tasks are ordered by due date
    Expected Result: Tasks reorder by due date
    Failure Indicators: Order unchanged, wrong sort field applied
    Evidence: .sisyphus/evidence/task-12-sort-due.png

  Scenario: Filters persist across page reload
    Tool: Playwright (playwright skill)
    Preconditions: Frontend running
    Steps:
      1. Navigate to http://localhost:5173
      2. Apply status filter "pending" and sort "priority"
      3. Assert URL shows `?status=pending&sort_by=priority`
      4. Reload the page
      5. Assert same filters are active after reload
      6. Assert task list reflects the filters
    Expected Result: Filters survive reload via URL params
    Failure Indicators: Filters reset, URL params lost
    Evidence: .sisyphus/evidence/task-12-filter-persistence.png
  ```

  **Evidence to Capture:**
  - [ ] task-12-filter-status.png
  - [ ] task-12-sort-due.png
  - [ ] task-12-filter-persistence.png

  **Commit**: YES (groups with T10, T11)
  - Message: (included in T10's commit)
  - Pre-commit: `pnpm build --filter frontend`

- [ ] 13. Keyboard Navigation System

  **What to do**:
  - Create `frontend/src/hooks/useKeyboardNav.ts`: Custom hook for keyboard navigation
  - **Navigation keys** (only active when task list is focused, NOT when in an input/form):
    - `j` or `ArrowDown`: Move selection down one row
    - `k` or `ArrowUp`: Move selection up one row
    - `g` then `g`: Jump to first task (vim gg)
    - `G` (shift+g): Jump to last task
  - **Action keys** (on currently selected task):
    - `Enter` or `e`: Enter edit mode (triggers TaskForm edit from T11)
    - `a`: Enter create mode (triggers TaskForm create from T11)
    - `c`: Complete selected task (calls `useCompleteTask()`)
    - `u`: Uncomplete selected task (calls `useUncompleteTask()`)
    - `s`: Start/stop toggle (start if not active, stop if active)
    - `d`: Delete selected task (with `window.confirm` prompt)
    - `/`: Focus the filter/search area (future-proofing, for now focus project input in filter bar)
    - `Escape`: Clear selection / exit edit mode / close modals
  - **Focus management**:
    - Task list is focusable (`tabIndex={0}`) — clicking it or pressing Tab brings focus
    - When an input is focused (create/edit form, filter inputs), keyboard shortcuts are DISABLED
    - Use `document.activeElement` check or `event.target` to determine context
    - After mutations (complete, delete, etc.), maintain selection position or move to next row
  - **Visual indicator**: Show a keyboard shortcut help hint at bottom of page or in status bar:
    - "j/k: navigate | Enter: edit | a: add | c: complete | s: start/stop | d: delete | ?: help"
  - **MUST prevent conflicts with browser shortcuts**:
    - Only `e.preventDefault()` for single-key shortcuts when task list is focused
    - NEVER intercept `Ctrl+*` or `Cmd+*` shortcuts

  **Must NOT do**:
  - Do NOT add a command palette (Cmd+K)
  - Do NOT add vim-style modal editing (normal/insert/visual modes beyond basic j/k)
  - Do NOT add custom key rebinding
  - Do NOT intercept browser shortcuts (Ctrl+C, Ctrl+V, Ctrl+D, etc.)

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Complex event handling with focus management, context-dependent key behavior, and edge cases
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `frontend-ui-ux`: This is primarily event handling logic, not visual design

  **Parallelization**:
  - **Can Run In Parallel**: YES (but depends on T10 for row selection)
  - **Parallel Group**: Wave 3 (with Tasks 10, 11, 12, 14)
  - **Blocks**: Task 16
  - **Blocked By**: Task 10

  **References**:

  **Pattern References**:
  - `frontend/src/components/TaskList.tsx` (from T10) — selected row state, row click handling
  - `frontend/src/components/TaskForm.tsx` (from T11) — create/edit mode triggers
  - `frontend/src/api/hooks.ts` (from T9) — mutation hooks for keyboard-triggered actions

  **External References**:
  - KeyboardEvent.key values: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values

  **WHY Each Reference Matters**:
  - TaskList's selected row state is what keyboard nav modifies — they share state
  - TaskForm's trigger functions are what keyboard shortcuts call — must match the interface

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Navigate task list with j/k and arrow keys
    Tool: Playwright (playwright skill)
    Preconditions: Server running with 5+ tasks, frontend running
    Steps:
      1. Navigate to http://localhost:5173
      2. Click on the task list to focus it
      3. Press 'j' key
      4. Assert second row is selected (has distinct styling)
      5. Press 'j' again
      6. Assert third row is selected
      7. Press 'k'
      8. Assert second row is selected again
      9. Press 'ArrowDown'
      10. Assert third row is selected
      11. Press 'ArrowUp'
      12. Assert second row is selected
    Expected Result: Both j/k and arrow keys navigate rows
    Failure Indicators: Selection doesn't move, wrong row highlighted, keys don't work
    Evidence: .sisyphus/evidence/task-13-jk-navigation.png

  Scenario: Action keys trigger correct operations
    Tool: Playwright (playwright skill)
    Preconditions: Server running with at least 1 pending task, frontend running
    Steps:
      1. Navigate to http://localhost:5173, focus task list
      2. Select first task with click
      3. Note the task description
      4. Press 'c' key
      5. Assert task status changes to completed (or task disappears from pending filter)
      6. Press 'u' key
      7. Assert task status returns to pending
      8. Press 's' key
      9. Assert task shows active indicator
      10. Press 's' again
      11. Assert active indicator removed
    Expected Result: c/u/s keys trigger complete/uncomplete/start-stop
    Failure Indicators: No status change, wrong action, API error
    Evidence: .sisyphus/evidence/task-13-action-keys.png

  Scenario: Keys disabled when input is focused
    Tool: Playwright (playwright skill)
    Preconditions: Frontend running with tasks
    Steps:
      1. Navigate to http://localhost:5173
      2. Click on a filter input (e.g., project filter)
      3. Press 'j' key
      4. Assert 'j' character appears in input (not captured by nav)
      5. Assert task selection does NOT change
    Expected Result: Keyboard shortcuts only work when task list is focused
    Failure Indicators: Shortcuts fire while typing in input
    Evidence: .sisyphus/evidence/task-13-input-focus.png
  ```

  **Evidence to Capture:**
  - [ ] task-13-jk-navigation.png
  - [ ] task-13-action-keys.png
  - [ ] task-13-input-focus.png

  **Commit**: YES
  - Message: `feat(frontend): add keyboard navigation system`
  - Files: `frontend/src/hooks/useKeyboardNav.ts`, `frontend/src/components/TaskList.tsx`
  - Pre-commit: `pnpm build --filter frontend`

- [ ] 14. Sync Config Modal

  **What to do**:
  - Create `frontend/src/components/SyncModal.tsx`: Modal/panel for sync configuration
  - **Trigger**: Button in header/status bar labeled "Sync" or gear icon
  - **Modal content**:
    - Section: "Sync Configuration"
      - Server URL input (text, placeholder: "https://sync.example.com")
      - Encryption Secret input (password type, placeholder: "your-secret-key")
      - Client ID input (text, placeholder: "uuid-v4")
      - "Save Configuration" button → calls `useSyncConfig().mutate()`
      - Success/error feedback inline
    - Section: "Actions"
      - "Sync Now" button → calls `useSync().mutate()` → shows SyncResult (success + message)
      - Loading state while syncing (disable all buttons, show spinner/text "Syncing...")
      - NOTE: Sync may block for several seconds — show clear feedback
    - Section: "Danger Zone" (red-styled)
      - "Clear Local Data" button → `window.confirm("This will delete all local task data. Are you sure?")` → calls `useClearData().mutate()`
  - **Modal behavior**:
    - Opens as an overlay (position fixed, centered, dark backdrop)
    - Closes on Escape key or clicking backdrop
    - Terminal-styled: monospace, bordered, dark/light theme-aware
  - Wire "Sync" button into Layout.tsx header

  **Must NOT do**:
  - Do NOT store sync credentials in localStorage (they go directly to the server)
  - Do NOT auto-sync on a timer
  - Do NOT validate sync URL format (let the server/core handle validation)
  - Do NOT install a modal library

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Modal UI with form inputs and action buttons — visual component
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: Modal design, form layout, danger zone styling
  - **Skills Evaluated but Omitted**:
    - `playwright`: QA is separate

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 10, 11, 12, 13)
  - **Blocks**: Task 16
  - **Blocked By**: Tasks 8, 9

  **References**:

  **Pattern References**:
  - `frontend/src/api/hooks.ts` (from T9) — `useSyncConfig()`, `useSync()`, `useClearData()` hooks
  - `frontend/src/api/types.ts` (from T9) — `SyncConfig`, `SyncResult` types
  - `frontend/src/components/Layout.tsx` (from T8) — header where sync button goes

  **API/Type References**:
  - `SyncConfig { server_url: string, encryption_secret: string, client_id: string }`
  - `SyncResult { success: boolean, message: string }`
  - Server endpoints: `POST /api/sync/configure`, `POST /api/sync`, `POST /api/data/clear`

  **WHY Each Reference Matters**:
  - `SyncResult.message` contains the sync outcome details — display it to user
  - `useClearData()` invalidates ALL queries — after clearing, the task list will be empty

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Sync modal opens and closes
    Tool: Playwright (playwright skill)
    Preconditions: Frontend running
    Steps:
      1. Navigate to http://localhost:5173
      2. Click "Sync" button in header
      3. Assert modal overlay appears (selector: `[data-testid="sync-modal"]` or similar)
      4. Assert Server URL, Encryption Secret, Client ID inputs are visible
      5. Assert "Sync Now" and "Clear Local Data" buttons exist
      6. Press Escape
      7. Assert modal is closed
    Expected Result: Modal opens/closes correctly
    Failure Indicators: Modal doesn't appear, inputs missing, Escape doesn't close
    Evidence: .sisyphus/evidence/task-14-sync-modal.png

  Scenario: Sync without configuration shows error
    Tool: Playwright (playwright skill)
    Preconditions: Frontend and server running, sync NOT configured
    Steps:
      1. Open sync modal
      2. Click "Sync Now" button
      3. Assert error message appears (sync not configured)
      4. Assert error is displayed in the modal, not as a page crash
    Expected Result: Graceful error message about sync not being configured
    Failure Indicators: Unhandled error, page crash, no feedback
    Evidence: .sisyphus/evidence/task-14-sync-error.png

  Scenario: Clear data with confirmation
    Tool: Playwright (playwright skill)
    Preconditions: Frontend and server running, at least 1 task exists
    Steps:
      1. Open sync modal
      2. Click "Clear Local Data"
      3. Assert confirmation dialog appears
      4. Accept confirmation
      5. Assert task list is now empty (tasks were cleared)
      6. Close modal
      7. Assert main view shows empty state
    Expected Result: Data cleared after confirmation, task list empty
    Failure Indicators: No confirmation, data not cleared, tasks still visible
    Evidence: .sisyphus/evidence/task-14-clear-data.png
  ```

  **Evidence to Capture:**
  - [ ] task-14-sync-modal.png
  - [ ] task-14-sync-error.png
  - [ ] task-14-clear-data.png

  **Commit**: YES
  - Message: `feat(frontend): add sync configuration modal`
  - Files: `frontend/src/components/SyncModal.tsx`, `frontend/src/components/Layout.tsx`
  - Pre-commit: `pnpm build --filter frontend`

- [ ] 15. Server Tests (cargo test)

  **What to do**:
  - Create `server/tests/api_tests.rs`: Integration tests for all API endpoints
  - Use axum's test utilities (`axum::test` / `tower::ServiceExt`) to send requests without starting a real server
  - Each test creates a fresh `TaskManagerWrapper` with a temp directory (`tempfile::TempDir`) — isolated test data
  - **Test coverage** (one test function per endpoint group):
    - `test_health_and_version`: GET /api/health → 200, GET /api/version → 200 with version string
    - `test_create_task`: POST /api/tasks with valid body → 201 + TaskInfo
    - `test_create_task_empty_description`: POST /api/tasks with empty description → 400
    - `test_get_task`: Create a task, GET /api/tasks/:uuid → 200 + matching TaskInfo
    - `test_get_task_not_found`: GET /api/tasks/00000000-... → 404
    - `test_get_task_invalid_uuid`: GET /api/tasks/not-a-uuid → 400
    - `test_update_task`: Create task, PATCH with new priority/project → 200 + updated fields
    - `test_delete_task`: Create task, DELETE → 204, GET same uuid → 404
    - `test_list_tasks`: Create 3 tasks, GET /api/tasks → 200 + 3 items
    - `test_list_tasks_filter_status`: Create + complete a task, GET ?status=completed → only completed
    - `test_list_tasks_sort`: GET ?sort_by=description → alphabetical order
    - `test_list_tasks_invalid_sort`: GET ?sort_by=invalid → 400
    - `test_complete_uncomplete`: Complete → status=completed, Uncomplete → status=pending
    - `test_start_stop`: Start → is_active=true, Stop → is_active=false
    - `test_working_set`: Create tasks, GET /api/working-set → array with sequential IDs
    - `test_sync_not_configured`: POST /api/sync → 409
    - `test_configure_sync`: POST /api/sync/configure → 200 (can't fully test without sync server)
  - Add `tempfile` and `tower` as dev-dependencies in server/Cargo.toml
  - Add `serde_json` as dev-dependency if not already
  - Ensure all tests pass: `cargo test --manifest-path server/Cargo.toml`

  **Must NOT do**:
  - Do NOT mock the core — use real `TaskManagerWrapper` with temp directory
  - Do NOT test against a real running server process (use tower test utilities)
  - Do NOT add benchmarks or performance tests
  - Do NOT test sync with a real sync server

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Many test cases covering all endpoints — needs thorough understanding of API surface
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - None relevant for Rust test writing

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 16, 17)
  - **Blocks**: Final wave
  - **Blocked By**: Tasks 4, 5, 6, 7

  **References**:

  **Pattern References**:
  - `server/src/handlers.rs` (from T4-T7) — all handler functions and route registrations
  - `server/src/main.rs` — router setup and app factory (may need to extract `create_app()` for testing)
  - `server/src/types.rs` (from T3) — request body shapes for POST/PATCH requests

  **External References**:
  - axum testing: https://docs.rs/axum/latest/axum/index.html#testing — `Router::oneshot()` pattern
  - tower ServiceExt: https://docs.rs/tower/latest/tower/trait.ServiceExt.html

  **WHY Each Reference Matters**:
  - May need to extract router creation into a `create_app(state: AppState) -> Router` function to reuse in tests
  - `Router::oneshot(request)` sends a single request without starting a server — ideal for integration tests

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: All server tests pass
    Tool: Bash
    Preconditions: Server code complete (T1-T7)
    Steps:
      1. Run `cargo test --manifest-path server/Cargo.toml -- --nocapture`
      2. Assert exit code is 0
      3. Assert output shows 17+ tests passing
      4. Assert 0 test failures
    Expected Result: All tests pass
    Failure Indicators: Test failures, compilation errors, panics
    Evidence: .sisyphus/evidence/task-15-cargo-test.txt

  Scenario: Tests are independent (no shared state)
    Tool: Bash
    Preconditions: Tests written
    Steps:
      1. Run `cargo test --manifest-path server/Cargo.toml -- --test-threads=1`
      2. Assert all pass (sequential execution)
      3. Run `cargo test --manifest-path server/Cargo.toml` (default parallel)
      4. Assert all pass (parallel execution)
    Expected Result: Tests pass both sequentially and in parallel
    Failure Indicators: Tests fail only in parallel (shared state issue)
    Evidence: .sisyphus/evidence/task-15-test-isolation.txt
  ```

  **Evidence to Capture:**
  - [ ] task-15-cargo-test.txt
  - [ ] task-15-test-isolation.txt

  **Commit**: YES
  - Message: `test(server): add API endpoint tests`
  - Files: `server/tests/`, `server/Cargo.toml`
  - Pre-commit: `cargo test --manifest-path server/Cargo.toml`

- [ ] 16. Frontend Tests (Vitest)

  **What to do**:
  - Install MSW (Mock Service Worker): `pnpm add -D msw` in frontend/
  - Create `frontend/src/test/setup.ts`: Vitest setup file — import testing-library/jest-dom matchers
  - Create `frontend/src/test/mocks/handlers.ts`: MSW handlers mocking all API endpoints
    - GET /api/tasks → returns mock TaskInfo array
    - POST /api/tasks → returns mock created TaskInfo
    - PATCH /api/tasks/:uuid → returns mock updated TaskInfo
    - DELETE /api/tasks/:uuid → returns 204
    - POST /api/tasks/:uuid/complete → returns mock TaskInfo with status=completed
    - etc. for all endpoints
  - Create `frontend/src/test/mocks/server.ts`: MSW `setupServer()` instance
  - **Test files**:
    - `frontend/src/api/__tests__/client.test.ts`: Test API client functions
      - Test that `createTask` sends POST with correct body
      - Test that `listTasks` sends GET with correct query params
      - Test that API errors are properly thrown as `ApiError`
    - `frontend/src/api/__tests__/hooks.test.tsx`: Test TanStack Query hooks
      - Test `useTaskList()` returns data from mock
      - Test `useCreateTask()` invalidates task list cache
    - `frontend/src/components/__tests__/TaskList.test.tsx`: Test TaskList component
      - Renders task list with mock data
      - Shows empty state when no tasks
      - Shows loading state initially
    - `frontend/src/components/__tests__/FilterBar.test.tsx`: Test FilterBar
      - Filter dropdown changes trigger re-fetch with params
    - `frontend/src/theme/__tests__/ThemeContext.test.tsx`: Test theme toggle
      - Default is dark
      - Toggle switches to light
      - Persists in localStorage
  - Configure vitest in `frontend/vite.config.ts` or `frontend/vitest.config.ts`:
    - environment: 'jsdom'
    - setupFiles: './src/test/setup.ts'
    - globals: true
  - All tests pass: `pnpm test --filter frontend`

  **Must NOT do**:
  - Do NOT test implementation details (internal state, private functions)
  - Do NOT add E2E tests here (that's the Final Verification wave)
  - Do NOT test TanStack Query internals — test behavior through components
  - Do NOT mock `fetch` directly — use MSW

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Test setup with MSW + multiple test files covering different layers
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `frontend-ui-ux`: Tests, not visual work

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 15, 17)
  - **Blocks**: Final wave
  - **Blocked By**: Tasks 9, 10, 11, 12, 13, 14

  **References**:

  **Pattern References**:
  - `frontend/src/api/client.ts` (from T9) — API functions to test
  - `frontend/src/api/hooks.ts` (from T9) — hooks to test
  - `frontend/src/components/TaskList.tsx` (from T10) — component to test
  - `frontend/src/theme/ThemeContext.tsx` (from T8) — theme logic to test

  **External References**:
  - MSW docs: https://mswjs.io/docs/getting-started — setup and handlers
  - React Testing Library: https://testing-library.com/docs/react-testing-library/intro
  - Vitest setup: https://vitest.dev/guide/

  **WHY Each Reference Matters**:
  - MSW intercepts at the network level — tests the full fetch→parse→hook chain
  - React Testing Library tests user-visible behavior — matches the terminal UX focus

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: All frontend tests pass
    Tool: Bash
    Preconditions: Frontend code complete (T2, T8-T14)
    Steps:
      1. Run `pnpm test --filter frontend -- --run`
      2. Assert exit code is 0
      3. Assert output shows all test suites passing
      4. Assert 0 failures
    Expected Result: All tests pass
    Failure Indicators: Test failures, import errors, MSW setup issues
    Evidence: .sisyphus/evidence/task-16-vitest.txt

  Scenario: Tests cover core functionality
    Tool: Bash
    Preconditions: Tests written
    Steps:
      1. Run `pnpm test --filter frontend -- --run --reporter=verbose`
      2. Assert test output includes: TaskList, FilterBar, ThemeContext, API client test descriptions
      3. Assert at least 10 test cases total
    Expected Result: Broad coverage of core functionality
    Failure Indicators: Only 1-2 tests, missing component tests
    Evidence: .sisyphus/evidence/task-16-test-coverage.txt
  ```

  **Evidence to Capture:**
  - [ ] task-16-vitest.txt
  - [ ] task-16-test-coverage.txt

  **Commit**: YES
  - Message: `test(frontend): add component and hook tests`
  - Files: `frontend/src/test/`, `frontend/src/**/__tests__/`, `frontend/package.json`
  - Pre-commit: `pnpm test --filter frontend -- --run`

- [ ] 17. Production Build Config (Serve Static Files from Rust)

  **What to do**:
  - Modify `server/src/main.rs` to serve the frontend's built static files in production:
    - Add `tower_http::services::ServeDir` to serve `frontend/dist/` (or configurable path)
    - API routes take priority: `/api/*` → axum handlers
    - Everything else → serve from static files directory
    - SPA fallback: if a static file is not found, serve `index.html` (for client-side routing)
  - Use an environment variable `TASKGENERAL_STATIC_DIR` to configure the static files path (default: `./frontend/dist`)
  - Only serve static files if the directory exists (graceful degradation — dev mode doesn't need it)
  - Add `tower-http` `ServeDir` and `ServeFile` features to Cargo.toml if not already present
  - Update `tower-http` features in Cargo.toml: add `fs` feature for file serving
  - Test: `pnpm build` in frontend/ → `cargo run` → browser opens `http://localhost:8080` → full app works from single binary

  **Must NOT do**:
  - Do NOT embed static files in the binary (use `include_dir!` macro) — serve from filesystem
  - Do NOT add gzip compression middleware (keep it simple for local use)
  - Do NOT add cache headers (local dev tool, no caching needed)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Well-documented tower-http pattern, small change to main.rs
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 15, 16)
  - **Blocks**: Final wave
  - **Blocked By**: Tasks 1, 2

  **References**:

  **Pattern References**:
  - `server/src/main.rs` (from T1) — existing router setup where to add fallback service

  **External References**:
  - tower-http ServeDir: https://docs.rs/tower-http/latest/tower_http/services/struct.ServeDir.html
  - axum fallback: https://docs.rs/axum/latest/axum/struct.Router.html#method.fallback_service

  **WHY Each Reference Matters**:
  - `Router::fallback_service(ServeDir::new("path"))` is the pattern — API routes first, then static files
  - SPA fallback (serve index.html for unknown paths) is needed for client-side routing

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Single binary serves full application
    Tool: Playwright (playwright skill)
    Preconditions: `pnpm build` completed in frontend/, dist/ exists
    Steps:
      1. Set `TASKGENERAL_STATIC_DIR=frontend/dist`
      2. Start server: `cargo run --manifest-path server/Cargo.toml`
      3. Navigate to http://localhost:8080 (NOT :5173)
      4. Assert page loads with TaskGeneral heading
      5. Assert API calls work (task list loads)
      6. Take screenshot
    Expected Result: Full app served from single Rust binary
    Failure Indicators: 404 for static files, blank page, API errors
    Evidence: .sisyphus/evidence/task-17-production-build.png

  Scenario: Server works without static files (dev mode)
    Tool: Bash
    Preconditions: No frontend/dist/ directory
    Steps:
      1. Ensure frontend/dist does not exist (or set TASKGENERAL_STATIC_DIR to nonexistent path)
      2. Start server: `cargo run --manifest-path server/Cargo.toml`
      3. `curl -s http://localhost:8080/api/health`
      4. Assert API still works (200)
      5. `curl -s -w '%{http_code}' http://localhost:8080/`
      6. Assert 404 (no static files to serve — expected)
    Expected Result: API works without static files, root returns 404 gracefully
    Failure Indicators: Server crashes without static dir, API broken
    Evidence: .sisyphus/evidence/task-17-no-static.txt
  ```

  **Evidence to Capture:**
  - [ ] task-17-production-build.png
  - [ ] task-17-no-static.txt

  **Commit**: YES
  - Message: `feat(server): serve frontend static files in production`
  - Files: `server/src/main.rs`, `server/Cargo.toml`
  - Pre-commit: `cargo build --release --manifest-path server/Cargo.toml`

---

## Final Verification Wave

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, curl endpoint, run command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo clippy` in server/ + `pnpm lint` in frontend/ + `cargo test` + `pnpm test`. Review all changed files for: `as any`/`@ts-ignore`, empty catches, console.log in prod, commented-out code, unused imports. Check AI slop: excessive comments, over-abstraction, generic names (data/result/item/temp). Verify every axum handler uses `spawn_blocking`.
  Output: `Build [PASS/FAIL] | Lint [PASS/FAIL] | Tests [N pass/N fail] | Files [N clean/N issues] | VERDICT`

- [ ] F3. **Real Manual QA** — `unspecified-high` (+ `playwright` skill)
  Start from clean state. Start server (`cargo run` in server/), start frontend (`pnpm dev` in frontend/). Execute EVERY QA scenario from EVERY task — follow exact steps, capture evidence. Test cross-task integration: create task → filter by project → sort by due → start task → stop task → complete → uncomplete → delete. Test keyboard navigation end-to-end. Test theme toggle. Test sync config (without actual sync server). Save to `.sisyphus/evidence/final-qa/`.
  Output: `Scenarios [N/N pass] | Integration [N/N] | Edge Cases [N tested] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git log/diff). Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance. Detect cross-task contamination: Task N touching Task M's files. Flag unaccounted changes. Specifically check: no auth code, no component libraries, no SSR, no CORS middleware, no extra state libraries.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

| Group | Commit Message | Files | Pre-commit Check |
|-------|---------------|-------|-----------------|
| T1 | `feat(server): scaffold axum server with health check` | server/ | `cargo check` |
| T2 | `feat(frontend): scaffold React + Vite + Tailwind project` | frontend/ | `pnpm build` |
| T3 | `feat(server): add DTO types and error handling` | server/src/ | `cargo check` |
| T4+T5+T6+T7 | `feat(server): implement all REST API endpoints` | server/src/ | `cargo test` |
| T8 | `feat(frontend): add theme system and layout shell` | frontend/src/ | `pnpm build` |
| T9 | `feat(frontend): add API client and TanStack Query hooks` | frontend/src/ | `pnpm build` |
| T10+T11+T12 | `feat(frontend): add task list, inline editing, and filter controls` | frontend/src/ | `pnpm build` |
| T13 | `feat(frontend): add keyboard navigation system` | frontend/src/ | `pnpm build` |
| T14 | `feat(frontend): add sync configuration modal` | frontend/src/ | `pnpm build` |
| T15 | `test(server): add API endpoint tests` | server/src/, server/tests/ | `cargo test` |
| T16 | `test(frontend): add component and hook tests` | frontend/src/ | `pnpm test` |
| T17 | `feat(server): serve frontend static files in production` | server/ | `cargo build --release` |

---

## Success Criteria

### Verification Commands
```bash
# Server starts and responds
cargo run --manifest-path server/Cargo.toml &
sleep 3
curl -s http://localhost:8080/api/version  # Expected: {"version":"0.1.0"}

# Full CRUD cycle
UUID=$(curl -s -X POST http://localhost:8080/api/tasks -H 'Content-Type: application/json' -d '{"description":"Test task"}' | jq -r '.uuid')
curl -s http://localhost:8080/api/tasks/$UUID  # Expected: 200, TaskInfo
curl -s -X PATCH http://localhost:8080/api/tasks/$UUID -H 'Content-Type: application/json' -d '{"priority":"H"}'  # Expected: 200
curl -s -X POST http://localhost:8080/api/tasks/$UUID/start  # Expected: 200, is_active=true
curl -s -X POST http://localhost:8080/api/tasks/$UUID/stop  # Expected: 200
curl -s -X POST http://localhost:8080/api/tasks/$UUID/complete  # Expected: 200, status="completed"
curl -s -X DELETE http://localhost:8080/api/tasks/$UUID  # Expected: 204

# Server tests
cargo test --manifest-path server/Cargo.toml  # Expected: all pass

# Frontend builds
pnpm test --filter frontend  # Expected: all pass
pnpm build --filter frontend  # Expected: exit 0, dist/ created

# Frontend loads
# (Playwright: navigate to localhost:5173, assert task list renders)
```

### Final Checklist
- [ ] All "Must Have" present
- [ ] All "Must NOT Have" absent
- [ ] All server tests pass
- [ ] All frontend tests pass
- [ ] Frontend builds without errors
- [ ] Keyboard navigation works (vim + standard)
- [ ] Theme toggle works and persists
- [ ] Sync config UI functional
