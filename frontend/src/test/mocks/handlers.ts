import { http, HttpResponse } from 'msw'
import type { TaskInfo, WorkingSetItem } from '../../api/types'

const mockTask: TaskInfo = {
  uuid: '123e4567-e89b-12d3-a456-426614174000',
  description: 'Test task',
  status: 'pending',
  project: 'test-project',
  tags: ['test', 'frontend'],
  priority: 'M',
  entry: '2026-03-14T10:00:00Z',
  modified: '2026-03-14T10:00:00Z',
  urgency: 8.5,
  is_active: false,
  is_waiting: false,
}

const mockCompletedTask: TaskInfo = {
  ...mockTask,
  uuid: '223e4567-e89b-12d3-a456-426614174001',
  status: 'completed',
  description: 'Completed task',
}

export const handlers = [
  // List tasks
  http.get('/api/tasks', ({ request }) => {
    const url = new URL(request.url)
    const status = url.searchParams.get('status')
    
    if (status === 'completed') {
      return HttpResponse.json([mockCompletedTask])
    }
    
    return HttpResponse.json([mockTask])
  }),

  // Create task
  http.post('/api/tasks', async ({ request }) => {
    const body = await request.json() as { description: string }
    return HttpResponse.json({
      ...mockTask,
      uuid: '333e4567-e89b-12d3-a456-426614174002',
      description: body.description,
    })
  }),

  // Get task
  http.get('/api/tasks/:uuid', ({ params }) => {
    const { uuid } = params
    if (uuid === 'not-found') {
      return HttpResponse.json({ error: 'Task not found' }, { status: 404 })
    }
    return HttpResponse.json({ ...mockTask, uuid: uuid as string })
  }),

  // Update task
  http.put('/api/tasks/:uuid', async ({ params, request }) => {
    const { uuid } = params
    const updates = await request.json() as Record<string, unknown>
    return HttpResponse.json({
      ...mockTask,
      uuid: uuid as string,
      ...updates,
    })
  }),

  // Delete task
  http.delete('/api/tasks/:uuid', () => {
    return HttpResponse.json(null, { status: 204 })
  }),

  // Complete task
  http.post('/api/tasks/:uuid/complete', ({ params }) => {
    const { uuid } = params
    return HttpResponse.json({
      ...mockTask,
      uuid: uuid as string,
      status: 'completed',
    })
  }),

  // Uncomplete task
  http.post('/api/tasks/:uuid/uncomplete', ({ params }) => {
    const { uuid } = params
    return HttpResponse.json({
      ...mockTask,
      uuid: uuid as string,
      status: 'pending',
    })
  }),

  // Start task
  http.post('/api/tasks/:uuid/start', ({ params }) => {
    const { uuid } = params
    return HttpResponse.json({
      ...mockTask,
      uuid: uuid as string,
      is_active: true,
    })
  }),

  // Stop task
  http.post('/api/tasks/:uuid/stop', () => {
    return HttpResponse.json(null, { status: 200 })
  }),

  // Configure sync
  http.post('/api/sync/configure', () => {
    return HttpResponse.json({ status: 'configured' })
  }),

  // Sync
  http.post('/api/sync', () => {
    return HttpResponse.json(
      { error: 'Sync not configured' },
      { status: 409 }
    )
  }),

  // Clear data
  http.delete('/api/data', () => {
    return HttpResponse.json({ status: 'cleared' })
  }),

  // Get version
  http.get('/api/version', () => {
    return HttpResponse.json({ version: '1.0.0' })
  }),

  // Get working set
  http.get('/api/tasks/working-set', () => {
    const workingSet: WorkingSetItem[] = [
      { id: 1, task: mockTask },
    ]
    return HttpResponse.json(workingSet)
  }),
]
