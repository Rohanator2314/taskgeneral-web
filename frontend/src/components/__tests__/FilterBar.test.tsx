import { describe, it, expect, vi } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import FilterBar from '../FilterBar'

describe('FilterBar', () => {
  it('renders without crashing', () => {
    render(<FilterBar filter={{}} onFilterChange={() => {}} />)
    expect(screen.getByTitle('Filter by Status')).toBeInTheDocument()
  })

  it('status filter select element is present in DOM', () => {
    render(<FilterBar filter={{}} onFilterChange={() => {}} />)
    const select = screen.getByTitle('Filter by Status')
    expect(select).toBeInTheDocument()
    expect(select.tagName).toBe('SELECT')
  })

  it('changing filter value calls onFilterChange callback', () => {
    const onFilterChange = vi.fn()
    render(<FilterBar filter={{}} onFilterChange={onFilterChange} />)

    const select = screen.getByTitle('Filter by Status')
    fireEvent.change(select, { target: { value: 'completed' } })

    expect(onFilterChange).toHaveBeenCalledWith({ status: 'completed' })
  })

  it('clear button appears when a filter is active', () => {
    render(<FilterBar filter={{ status: 'pending' }} onFilterChange={() => {}} />)
    expect(screen.getByTitle('Clear all filters')).toBeInTheDocument()
  })

  it('clear button calls onFilterChange with empty object', () => {
    const onFilterChange = vi.fn()
    render(<FilterBar filter={{ status: 'pending' }} onFilterChange={onFilterChange} />)

    fireEvent.click(screen.getByTitle('Clear all filters'))
    expect(onFilterChange).toHaveBeenCalledWith({})
  })
})
