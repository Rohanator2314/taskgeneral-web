# Contributing to taskgeneral-web

## Prerequisites

- Rust (stable, 2021 edition)
- Node.js 18+
- pnpm

## Development Setup

```bash
# Terminal 1 — API server
cd server
cargo run
# Listens on http://localhost:8080

# Terminal 2 — frontend dev server
cd frontend
pnpm install
pnpm dev
# Opens http://localhost:5173 (proxies /api → localhost:8080)
```

## Running Tests

```bash
# Server — Rust integration tests (real TaskManagerWrapper, no mocks)
cd server
cargo test -- --test-threads=1

# Frontend — Vitest + Testing Library + MSW
cd frontend
pnpm test
```

All tests must pass before submitting a PR.

## Code Style

**Rust**: `cargo fmt` and `cargo clippy -- -D warnings` must pass clean.

**TypeScript/React**:
- No component libraries (shadcn/ui, Radix, MUI, etc.) — all UI is hand-rolled
- No global state beyond React context
- No `as any`, `@ts-ignore`, or `@ts-expect-error`
- Run `pnpm lint` from the frontend directory before committing

## Submitting a Pull Request

1. Fork the repo and create a branch from `main`
2. Make your changes with focused, atomic commits
3. Add or update tests for any code you change
4. Ensure `cargo test -- --test-threads=1` and `pnpm test` both pass
5. Open a PR with the title format: `[taskgeneral-web] <short description>`

## Reporting Issues

Open a GitHub issue with:
- A clear description of the problem or feature request
- Steps to reproduce (for bugs)
- Your OS, Rust version (`rustc --version`), and Node version (`node --version`)
