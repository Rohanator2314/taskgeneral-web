import { describe, it, expect } from 'vitest'
import { render, screen, waitFor } from '@testing-library/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { type ReactNode } from 'react'
import { server } from '../../test/mocks/server'
import { http, HttpResponse } from 'msw'
import TaskList from '../TaskList'

function renderWithProviders(ui: ReactNode) {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  })
  return render(
    <QueryClientProvider client={queryClient}>{ui}</QueryClientProvider>
  )
}

describe('TaskList', () => {
  it('renders task description when mock has tasks', async () => {
    renderWithProviders(<TaskList />)

    await waitFor(() => {
      expect(screen.getByText('Test task')).toBeInTheDocument()
    })
  })

  it('shows empty state when no tasks', async () => {
    server.use(
      http.get('/api/tasks', () => HttpResponse.json([]))
    )

    renderWithProviders(<TaskList />)

    await waitFor(() => {
      expect(screen.getByText(/no tasks/i)).toBeInTheDocument()
    })
  })

  it('shows loading state initially', () => {
    renderWithProviders(<TaskList />)
    expect(screen.getByText(/loading/i)).toBeInTheDocument()
  })
})
