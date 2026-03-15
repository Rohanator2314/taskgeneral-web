export interface TaskInfo {
  uuid: string;
  description: string;
  status: string;
  project?: string;
  tags: string[];
  priority?: string;
  entry?: string;
  modified?: string;
  due?: string;
  wait?: string;
  start?: string;
  recur?: string;
  urgency: number;
  is_active: boolean;
  is_waiting: boolean;
}

export interface TaskUpdateParams {
  description?: string;
  project?: string;
  tags?: string[];
  priority?: string;
  due?: string;
  wait?: string;
  recur?: string;
}

export interface TaskFilterParams {
  status?: string;
  project?: string;
  tag?: string;
  sort_by?: string;
}

export interface SyncConfig {
  server_url: string;
  encryption_secret: string;
  client_id: string;
}

export interface SyncResult {
  success: boolean;
  message: string;
}

export interface WorkingSetItem {
  id: number;
  task: TaskInfo;
}

export interface ApiError {
  error: string;
}
