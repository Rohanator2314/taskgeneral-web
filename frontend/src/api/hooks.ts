import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import type { TaskFilterParams, TaskUpdateParams, SyncConfig } from './types';
import * as client from './client';

export function useTaskList(filter?: TaskFilterParams) {
  return useQuery({
    queryKey: ['tasks', filter],
    queryFn: () => client.listTasks(filter),
  });
}

export function useTask(uuid: string) {
  return useQuery({
    queryKey: ['task', uuid],
    queryFn: () => client.getTask(uuid),
  });
}

export function useWorkingSet() {
  return useQuery({
    queryKey: ['working-set'],
    queryFn: () => client.getWorkingSet(),
  });
}

export function useVersion() {
  return useQuery({
    queryKey: ['version'],
    queryFn: () => client.getVersion(),
    staleTime: Infinity,
  });
}

export function useCreateTask() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (description: string) => client.createTask(description),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });
}

export function useUpdateTask() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ uuid, updates }: { uuid: string; updates: TaskUpdateParams }) =>
      client.updateTask(uuid, updates),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });
}

export function useDeleteTask() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (uuid: string) => client.deleteTask(uuid),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });
}

export function useCompleteTask() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (uuid: string) => client.completeTask(uuid),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });
}

export function useUncompleteTask() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (uuid: string) => client.uncompleteTask(uuid),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });
}

export function useStartTask() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (uuid: string) => client.startTask(uuid),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });
}

export function useStopTask() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (uuid: string) => client.stopTask(uuid),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });
}

export function useSyncConfig() {
  return useMutation({
    mutationFn: (config: SyncConfig) => client.configureSyncServer(config),
  });
}

export function useSync() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: () => client.syncNow(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });
}

export function useClearData() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: () => client.clearData(),
    onSuccess: () => {
      queryClient.invalidateQueries();
    },
  });
}
