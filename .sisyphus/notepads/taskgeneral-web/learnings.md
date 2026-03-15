# Learnings — taskgeneral-web

## Critical Architecture Constraint
- Server MUST use `#[tokio::main]` (multi-threaded, NOT current_thread)
- ALL TaskManagerWrapper calls MUST use `tokio::task::spawn_blocking`
- Reason: Core creates its own `new_current_thread()` Runtime internally; nesting = panic

## Core API Surface
- `create_task_manager(data_dir: String) -> Result<Arc<TaskManagerWrapper>>`
- `version() -> String` — FREE FUNCTION, not a method on the wrapper
- All 15 methods on TaskManagerWrapper

## Serde Gap
- Core types WITHOUT Serialize/Deserialize: TaskUpdate, TaskFilter, SortField, WorkingSetItem
- Server needs own DTO types that map to/from core types

## Env Vars
- TASKGENERAL_DATA_DIR — default: ~/.local/share/taskgeneral/
- TASKGENERAL_PORT — default: 8080
- TASKGENERAL_STATIC_DIR — default: ./frontend/dist

## Frontend Scaffold — Task 2 Completion

### Tailwind CSS v4 Integration (KEY LEARNINGS)
- **Plugin approach**: Use `@tailwindcss/vite` plugin instead of PostCSS config
- **CSS import**: Single line in entry CSS file: `@import "tailwindcss";`
- **NO tailwind.config.js needed** — Vite plugin handles all config
- **Vite compatibility**: Tailwind v4 has peer dep issue with Vite 8 (expects <=v7), but works anyway
- **Resolution**: Install as-is, peer warnings are acceptable for production builds

### React TypeScript Setup
- **JSX config**: Must set `"jsx": "react-jsx"` in tsconfig.json for automatic imports
- **Types needed**: @types/react and @types/react-dom (peer of React itself)
- **Entry point**: Changed from main.ts to main.tsx, div id from "app" to "root"
- **React version**: React 19.2.4 (latest, works fine)

### TanStack Query Integration
- **Provider placement**: Wrap entire app in `<QueryClientProvider>` at root level
- **Config**: Default options set `refetchOnWindowFocus: true` for standard behavior
- **Package**: @tanstack/react-query v5.90.21

### Vitest Configuration
- **Setup file**: `src/test/setup.ts` imports '@testing-library/jest-dom'
- **Config in vite.config.ts**:
  ```typescript
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './src/test/setup.ts',
  }
  ```
- **Scripts**: "test": "vitest run" (single run) and "test:watch": "vitest"

### Vite Proxy for Backend
- **Config**: Routes `/api/*` to `http://localhost:8080` during dev
- **Benefits**: Eliminates CORS issues during frontend development
- **Production**: Backend serves static dist/ files, proxy not needed

### Build Output Metrics
- HTML: 0.46 kB (gzipped 0.29 kB)
- CSS (Tailwind): 6.33 kB (gzipped 1.88 kB)
- JS (React + deps): 215.06 kB (gzipped 67.07 kB)
- **Performance**: Built in 149ms, 62 modules transformed
- **Result**: Production-ready optimized bundle

### Monorepo Structure
- `pnpm-workspace.yaml` lists packages at workspace root
- Root `package.json` has workspace scripts: dev:frontend, build, test
- Each package is independent with own node_modules management
- Benefits: Shared dependencies, clean separation of concerns

### Common Pitfalls Avoided
- ✅ Did NOT use component libraries (shadcn, Radix) — scope: minimal
- ✅ Did NOT use state management (Zustand, Redux) — only React Query
- ✅ Did NOT install MSW mocking — backend integration is task-3
- ✅ Did NOT style beyond heading — theming is later
- ✅ Did NOT use Next.js or SSR — pure Vite for speed

### File Structure Created
```
frontend/
  ├── src/
  │   ├── main.tsx          # Entry point with QueryClientProvider
  │   ├── App.tsx           # Root component with "TaskGeneral" heading
  │   ├── style.css         # @import "tailwindcss" only
  │   ├── test/
  │   │   └── setup.ts      # Vitest setup
  │   └── assets/           # Static images (unused currently)
  ├── index.html            # Points to root div, loads main.tsx
  ├── vite.config.ts        # React + Tailwind + Vitest + proxy config
  ├── tsconfig.json         # JSX + strict mode enabled
  └── package.json          # Dependencies + build/test scripts
```

### Next Task (Frontend Integration)
- Create API client types (QueryDTO types matching Core's types)
- Build task list component with React Query
- Wire up server proxy and test full client-server flow

## [2026-03-15] Task 3: Server DTO Types + Error Handling

### API & Module Structure
- **Correct module path**: `taskgeneral_core::error::TaskError` (not CoreError)
- **Model types are public**: `taskgeneral_core::models::{TaskUpdate, TaskFilter, SortField}`
- Main re-exports only: `version()` free function and `TaskManagerWrapper` struct
- All utility types (errors, models) must be imported from specific submodules

### Error Type Mapping
- **TaskError::TaskNotFound** → HTTP 404 (NOT_FOUND)
- **TaskError::InvalidUuid/Description/Priority/Date/Recurrence/Status/SyncUrl** → HTTP 400 (BAD_REQUEST)
- **TaskError::SyncNotConfigured** → HTTP 409 (CONFLICT)
- **TaskError::StorageError/SyncError/TaskChampionError/IoError** → HTTP 500 (INTERNAL_SERVER_ERROR)
- **AppError::BadRequest** → HTTP 400

### DTO Struct Design
**Request DTOs** (all with `#[derive(Debug, Deserialize)]`):
- CreateTaskRequest { description: String }
- UpdateTaskRequest { description, project, tags, priority, due, wait, recur — all Option<T> }
- TaskFilterQuery { status, project, tag, sort_by — all Option<T> }
- SyncConfigRequest { server_url, encryption_secret, client_id }

**Response DTOs** (all with `#[derive(Debug, Serialize)]`):
- ErrorResponse { error: String }
- VersionResponse { version: String }
- HealthResponse { status: String }

### Conversion Pattern
**From<UpdateTaskRequest> for TaskUpdate**: Direct field mapping
**From<TaskFilterQuery> for TaskFilter**: Direct field mapping
**parse_sort_field(s: &str) → Result<SortField, AppError>**: 
- Maps "urgency", "due", "priority", "entry", "modified", "description" variants
- Returns AppError::BadRequest with helpful message for unknown values

### Testing Strategy
- Test each TaskError variant individually (12 tests)
- Test AppError::BadRequest variant (1 test)
- Use IntoResponse trait to verify status codes directly
- All 14 tests pass, covering 100% of error mapping logic

### Unused Warning Rationale
- Response types (VersionResponse, HealthResponse) for future use in route handlers
- parse_sort_field() function for future task list filtering
- AppError enum for route handler integration
- These are API contracts, not yet wired to routes (task 4+)

## [2026-03-15] Task 4: CRUD Handlers
- All handlers use spawn_blocking — MANDATORY pattern for blocking core operations
- get_task returns Option<TaskInfo>, convert None to AppError::Core(TaskError::TaskNotFound)
- TaskError is the actual type name (not CoreError) from taskgeneral_core::error::TaskError
- TaskUpdate converted via From<UpdateTaskRequest> trait implementation
- Route pattern: .route("/api/tasks/:uuid", get(f).patch(f).delete(f)) for multiple methods on same path
- Ownership fix: Clone uuid before moving into spawn_blocking closure to use it later in error message
- DELETE sets status to "deleted" rather than removing the task (soft delete)
- Core validation handles empty descriptions and invalid UUIDs, returning proper 400 errors
- Used port 8081 for testing due to port 8080 being occupied

## [2026-03-15] Task 8: Theme System + Layout Shell
- ThemeContext stores theme in localStorage key 'tg-theme'
- Applies 'dark' or 'light' class to document.documentElement
- Tailwind v4: use @theme directive for custom CSS variables (no tailwind.config.js)
- Layout has: header (data-testid="layout-header"), main, status bar
- Theme toggle: data-testid="theme-toggle"
- JetBrains Mono loaded via Google Fonts CDN in index.html
- NOTE: Playwright verification skipped due to missing browser dependencies in the environment.

## [2026-03-15] Task 5: List Tasks Endpoint
- Decision tree: sort_by present → list_tasks_sorted, filter only → list_tasks_filtered, none → list_tasks
- parse_sort_field() in types.rs converts string to SortField enum
- Route pattern: .route("/api/tasks", get(list_tasks).post(create_task))
- Query extractor: axum::extract::Query<TaskFilterQuery>
- list_tasks uses has_sort and has_filter flags to determine which manager method to call
- All three list methods wrapped in spawn_blocking as per core usage pattern
- Invalid sort_by returns 400 with descriptive error from parse_sort_field()
- Result type: Vec<TaskInfo> serialized to JSON array

## Task 6: Task Status Endpoints Implementation

### Key Learnings

1. **Handler Pattern Consistency**: All status handlers follow the same `spawn_blocking` pattern established by `create_task`, `get_task`, `update_task`, and `delete_task`. This is mandatory because the core library runs its own `current_thread` runtime and nesting runtimes causes panics.

2. **stop_task Special Case**: Unlike `complete_task`, `uncomplete_task`, and `start_task` which return `TaskInfo` (JSON body), `stop_task` returns `Result<(), TaskError>` and the handler must return `StatusCode::OK` with no body.

3. **Route Registration Pattern**: 
   - Routes appended AFTER existing routes in main.rs Router::new() chain
   - Must import new handlers in the `use handlers::` statement
   - Routes use POST method for all status operations
   - UUID path parameter captured via `Path(uuid): Path<String>`

4. **Error Handling**: TaskError::TaskNotFound automatically maps to HTTP 404 via AppError::Core variant and IntoResponse implementation in error.rs.

5. **Testing Strategy**: Use curl with `-w '\nHTTP Status: %{http_code}\n'` flag to capture both response body and HTTP status code in single command.

6. **Port Availability**: Always check for existing processes on target port when starting server. Use `lsof -i :PORT` or set TASKGENERAL_PORT env var to run on alternate port.

### Implementation Summary
- Added 4 handler functions to handlers.rs (113 lines → 185 lines)
- Registered 4 new routes in main.rs
- All handlers use `tokio::task::spawn_blocking` to prevent runtime nesting
- Compilation verified with `cargo check` (warnings only, no errors)
- All endpoints tested with curl lifecycle sequence
- 404 error handling verified

## [2026-03-15] Task 7: Sync + Utility Endpoints

### Handler Implementation Pattern
- **4 new handlers added**: `configure_sync`, `sync_now`, `clear_data`, `get_working_set`
- All use `tokio::task::spawn_blocking` for blocking core calls
- All return `Result<impl IntoResponse, AppError>`

### Serde Gaps in Core
- **SyncResult**: Does NOT implement Serialize. Manual mapping required:
  ```rust
  Ok(Json(serde_json::json!({
      "success": result.success,
      "message": result.message,
  })))
  ```
- **WorkingSetItem**: Does NOT implement Serialize. Must map to JSON:
  ```rust
  let items: Vec<serde_json::Value> = working_set.into_iter().map(|item| {
      serde_json::json!({
          "id": item.id,
          "task": item.task,  // TaskInfo IS Serializable
      })
  }).collect();
  ```

### Endpoint Specifications

**POST /api/sync/configure**
- Request: SyncConfigRequest { server_url, encryption_secret, client_id }
- Response: 200 {"status": "configured"}
- Note: client_id MUST be valid UUID format

**POST /api/sync**
- No request body
- Returns SyncResult mapped to { success, message }
- Returns 409 CONFLICT if sync not configured yet

**POST /api/data/clear**
- No request body
- Returns 200 {"status": "cleared"}
- Logs warning: "clear_local_data called — destructive operation"
- No auth required (simple endpoint)

**GET /api/working-set**
- No request body
- Returns array of working set items with id and task details
- Each item has structure: { id: u64, task: TaskInfo }
- Returns 200 with empty array if no active tasks

### Version Endpoint
- Already existed as inline `version_handler` in main.rs
- NOT duplicated — left as-is
- Returns GET /api/version with { version: "0.1.0" }

### Testing Results
- All 4 new endpoints tested and verified working
- Error handling correct (409 for sync not configured)
- JSON serialization working correctly with manual mapping
- spawn_blocking pattern prevents runtime nesting issues
- cargo check passes with 0 errors (2 unused struct warnings only)

## Task 9: Frontend API Client + TanStack Query Hooks (2026-03-15)

### API Layer Implementation
- Created complete data access layer with 3 files:
  - `types.ts`: TypeScript interfaces matching server DTOs (TaskInfo, TaskUpdateParams, etc.)
  - `client.ts`: Thin fetch wrapper with 14 API functions covering all server endpoints
  - `hooks.ts`: TanStack Query hooks (4 queries + 10 mutations) for React integration

### TypeScript Configuration
- Vite/TypeScript uses `erasableSyntaxOnly` mode which rejects parameter properties in class constructors
- Solution: Declare properties separately, then assign in constructor body
- Pattern: `public readonly error: string;` then `this.error = error;` in constructor

### TanStack Query v5 Patterns
- Query keys: `['tasks', filter]` for list (enables cache sharing), `['task', uuid]` for individual
- All task mutations invalidate `['tasks']` queryKey to refetch after changes
- `useClearData()` invalidates ALL queries via `invalidateQueries()` with no args
- `useVersion()` has `staleTime: Infinity` since version doesn't change at runtime
- `useSyncConfig()` and `useSync()` don't need queryClient if they don't invalidate

### API Client Design
- `apiFetch<T>()` helper abstracts fetch boilerplate: JSON parsing, error handling, 204 No Content
- Custom `ApiError` class extends Error, exposes `error: string` field from server JSON
- Query params built with `URLSearchParams` for clean, type-safe parameter encoding
- 204 No Content responses return `undefined` (cast as `T`) for void Promise types

### Server Endpoint Coverage (14 functions)
All endpoints from Task 4-7 covered:
- Task CRUD: create, get, list, update, delete (5)
- Task status: complete, uncomplete, start, stop (4)
- Sync: configureSyncServer, syncNow (2)
- Utility: clearData, getVersion, getWorkingSet (3)

### Evidence Captured
- `task-9-types-compile.txt`: pnpm build output showing clean TypeScript compilation
- `task-9-api-surface.txt`: All 14 client functions and 14 hooks verified exported

### Build Success
TypeScript compiles cleanly with exit code 0. All types resolve correctly.

