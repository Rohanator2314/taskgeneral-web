-- Migration: 001_initial_schema.sql
-- Creates all tables needed for TaskChampion storage with multi-user support.
-- All tables include user_id for isolation.

-- Main tasks table (TaskChampion's UuidKeyedTodolist)
CREATE TABLE IF NOT EXISTS tasks (
    user_id UUID NOT NULL,
    uuid UUID NOT NULL,
    task BLOB NOT NULL,  -- Serialized taskchampion::Task
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    modified_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, uuid)
);

CREATE INDEX idx_tasks_user_id ON tasks(user_id);

-- Operations table (TaskChampion's OperationLog)
-- Stores all mutations for sync purposes
CREATE TABLE IF NOT EXISTS operations (
    user_id UUID NOT NULL,
    id SERIAL PRIMARY KEY,
    operation_type VARCHAR(50) NOT NULL,
    task_uuid UUID,
    task_data BLOB,  -- Serialized taskchampion::Task for task modifications
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_operations_user_id ON operations(user_id);
CREATE INDEX idx_operations_task_uuid ON operations(user_id, task_uuid);
CREATE INDEX idx_operations_timestamp ON operations(user_id, timestamp);

-- Task metadata (key-value store per task)
CREATE TABLE IF NOT EXISTS task_metadata (
    user_id UUID NOT NULL,
    task_uuid UUID NOT NULL,
    key VARCHAR(255) NOT NULL,
    value TEXT NOT NULL,
    PRIMARY KEY (user_id, task_uuid, key)
);

CREATE INDEX idx_task_metadata_user_task ON task_metadata(user_id, task_uuid);

-- Working set (active task UUIDs for quick access)
CREATE TABLE IF NOT EXISTS working_set (
    user_id UUID NOT NULL,
    task_uuid UUID NOT NULL,
    position INTEGER NOT NULL,
    PRIMARY KEY (user_id, task_uuid)
);

CREATE INDEX idx_working_set_user_position ON working_set(user_id, position);

-- Sync configuration per user
CREATE TABLE IF NOT EXISTS sync_config (
    user_id UUID PRIMARY KEY,
    server_url TEXT,
    client_id TEXT,
    encryption_secret_encrypted TEXT,  -- Encrypted at application level if needed
    last_sync TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    modified_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- User preferences (optional, for future use)
CREATE TABLE IF NOT EXISTS user_preferences (
    user_id UUID PRIMARY KEY,
    preferences JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    modified_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
