import { describe, it, expect } from 'vitest'
import { createTask, listTasks, getTask, ApiError } from '../client'

describe('API Client', () => {
  describe('createTask', () => {
    it('sends POST to /api/tasks with correct body and returns TaskInfo', async () => {
      const description = 'New test task'
      const result = await createTask(description)
      
      expect(result).toMatchObject({
        uuid: '333e4567-e89b-12d3-a456-426614174002',
        description: 'New test task',
        status: 'pending',
      })
      expect(result.tags).toBeInstanceOf(Array)
    })
  })

  describe('listTasks', () => {
    it('calls GET /api/tasks and returns array', async () => {
      const result = await listTasks()
      
      expect(Array.isArray(result)).toBe(true)
      expect(result).toHaveLength(1)
      expect(result[0]).toMatchObject({
        uuid: '123e4567-e89b-12d3-a456-426614174000',
        description: 'Test task',
        status: 'pending',
      })
    })

    it('includes status=completed in query params', async () => {
      const result = await listTasks({ status: 'completed' })
      
      expect(Array.isArray(result)).toBe(true)
      expect(result).toHaveLength(1)
      expect(result[0]).toMatchObject({
        status: 'completed',
        description: 'Completed task',
      })
    })
  })

  describe('getTask', () => {
    it('throws ApiError when server returns 404', async () => {
      await expect(getTask('not-found')).rejects.toThrow(ApiError)
      
      try {
        await getTask('not-found')
      } catch (error) {
        expect(error).toBeInstanceOf(ApiError)
        expect((error as ApiError).error).toBe('Task not found')
      }
    })

    it('returns task data for valid UUID', async () => {
      const result = await getTask('valid-uuid')
      
      expect(result).toMatchObject({
        uuid: 'valid-uuid',
        description: 'Test task',
      })
    })
  })
})
