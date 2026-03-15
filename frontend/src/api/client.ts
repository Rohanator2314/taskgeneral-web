import type {
  TaskInfo,
  TaskUpdateParams,
  TaskFilterParams,
  SyncConfig,
  SyncResult,
  WorkingSetItem,
} from './types';

const BASE_URL = '/api';

export class ApiError extends Error {
  public readonly error: string;
  
  constructor(error: string) {
    super(error);
    this.name = 'ApiError';
    this.error = error;
  }
}

async function apiFetch<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE_URL}${path}`, {
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    ...options,
  });
  
  if (!res.ok) {
    const body = await res.json().catch(() => ({ error: res.statusText }));
    throw new ApiError(body.error ?? res.statusText);
  }
  
  if (res.status === 204) return undefined as T;
  return res.json();
}

export async function createTask(description: string): Promise<TaskInfo> {
  return apiFetch<TaskInfo>('/tasks', {
    method: 'POST',
    body: JSON.stringify({ description }),
  });
}

export async function getTask(uuid: string): Promise<TaskInfo> {
  return apiFetch<TaskInfo>(`/tasks/${uuid}`);
}

export async function listTasks(filter?: TaskFilterParams): Promise<TaskInfo[]> {
  const params = new URLSearchParams();
  if (filter?.status) params.append('status', filter.status);
  if (filter?.project) params.append('project', filter.project);
  if (filter?.tag) params.append('tag', filter.tag);
  if (filter?.sort_by) params.append('sort_by', filter.sort_by);
  
  const queryString = params.toString();
  return apiFetch<TaskInfo[]>(`/tasks${queryString ? `?${queryString}` : ''}`);
}

export async function updateTask(
  uuid: string,
  updates: TaskUpdateParams
): Promise<TaskInfo> {
  return apiFetch<TaskInfo>(`/tasks/${uuid}`, {
    method: 'PUT',
    body: JSON.stringify(updates),
  });
}

export async function deleteTask(uuid: string): Promise<void> {
  return apiFetch<void>(`/tasks/${uuid}`, {
    method: 'DELETE',
  });
}

export async function completeTask(uuid: string): Promise<TaskInfo> {
  return apiFetch<TaskInfo>(`/tasks/${uuid}/complete`, {
    method: 'POST',
  });
}

export async function uncompleteTask(uuid: string): Promise<TaskInfo> {
  return apiFetch<TaskInfo>(`/tasks/${uuid}/uncomplete`, {
    method: 'POST',
  });
}

export async function startTask(uuid: string): Promise<TaskInfo> {
  return apiFetch<TaskInfo>(`/tasks/${uuid}/start`, {
    method: 'POST',
  });
}

export async function stopTask(uuid: string): Promise<void> {
  return apiFetch<void>(`/tasks/${uuid}/stop`, {
    method: 'POST',
  });
}

export async function configureSyncServer(config: SyncConfig): Promise<void> {
  await apiFetch<{ status: string }>('/sync/configure', {
    method: 'POST',
    body: JSON.stringify(config),
  });
}

export async function syncNow(): Promise<SyncResult> {
  return apiFetch<SyncResult>('/sync', {
    method: 'POST',
  });
}

export async function clearData(): Promise<void> {
  await apiFetch<{ status: string }>('/data', {
    method: 'DELETE',
  });
}

export async function getVersion(): Promise<string> {
  const response = await apiFetch<{ version: string }>('/version');
  return response.version;
}

export async function getWorkingSet(): Promise<WorkingSetItem[]> {
  return apiFetch<WorkingSetItem[]>('/tasks/working-set');
}
