-- Migration: 001_initial_schema.sql
-- Creates all tables needed for task storage with multi-user support.

CREATE TABLE IF NOT EXISTS tasks (
    user_id UUID NOT NULL,
    uuid UUID NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    project TEXT,
    tags TEXT[] NOT NULL DEFAULT '{}',
    priority TEXT,
    entry TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    modified_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    due TIMESTAMPTZ,
    wait TIMESTAMPTZ,
    start TIMESTAMPTZ,
    recur TEXT,
    urgency DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    is_active BOOLEAN NOT NULL DEFAULT false,
    is_waiting BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, uuid)
);

CREATE INDEX idx_tasks_user_id ON tasks(user_id);
CREATE INDEX idx_tasks_user_status ON tasks(user_id, status);
CREATE INDEX idx_tasks_user_project ON tasks(user_id, project);

CREATE TABLE IF NOT EXISTS working_set (
    user_id UUID NOT NULL,
    task_uuid UUID NOT NULL,
    position INTEGER NOT NULL,
    PRIMARY KEY (user_id, task_uuid)
);

CREATE INDEX idx_working_set_user_position ON working_set(user_id, position);

CREATE TABLE IF NOT EXISTS sync_config (
    user_id UUID PRIMARY KEY,
    server_url TEXT,
    client_id TEXT,
    encryption_secret_encrypted TEXT,
    last_sync TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    modified_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS user_preferences (
    user_id UUID PRIMARY KEY,
    preferences JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    modified_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
