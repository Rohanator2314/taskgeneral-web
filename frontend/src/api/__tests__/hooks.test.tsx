import { describe, it, expect } from 'vitest'
import { renderHook, waitFor } from '@testing-library/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { type ReactNode } from 'react'
import { useTaskList, useCreateTask } from '../hooks'

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  })
  return function Wrapper({ children }: { children: ReactNode }) {
    return (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    )
  }
}

describe('useTaskList', () => {
  it('returns task data from mock', async () => {
    const wrapper = createWrapper()
    const { result } = renderHook(() => useTaskList(), { wrapper })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    expect(Array.isArray(result.current.data)).toBe(true)
    expect(result.current.data).toHaveLength(1)
    expect(result.current.data![0]).toMatchObject({
      uuid: '123e4567-e89b-12d3-a456-426614174000',
      description: 'Test task',
    })
  })

  it('filters tasks when status param given', async () => {
    const wrapper = createWrapper()
    const { result } = renderHook(() => useTaskList({ status: 'completed' }), { wrapper })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    expect(result.current.data).toHaveLength(1)
    expect(result.current.data![0].status).toBe('completed')
  })
})

describe('useCreateTask', () => {
  it('mutation resolves with new task data', async () => {
    const wrapper = createWrapper()
    const { result } = renderHook(() => useCreateTask(), { wrapper })

    result.current.mutate('My new task')

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    expect(result.current.data).toMatchObject({
      description: 'My new task',
      uuid: '333e4567-e89b-12d3-a456-426614174002',
    })
  })
})
