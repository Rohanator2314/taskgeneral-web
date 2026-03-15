import { describe, it, expect, beforeEach } from 'vitest'
import { render, screen, fireEvent, act } from '@testing-library/react'
import { ThemeProvider, useTheme } from '../../theme/ThemeContext'

function ThemeConsumer() {
  const { theme, toggleTheme } = useTheme()
  return (
    <div>
      <span data-testid="theme-value">{theme}</span>
      <button onClick={toggleTheme}>Toggle</button>
    </div>
  )
}

describe('ThemeContext', () => {
  beforeEach(() => {
    localStorage.clear()
    document.documentElement.classList.remove('dark', 'light')
  })

  it('default theme is dark', () => {
    render(
      <ThemeProvider>
        <ThemeConsumer />
      </ThemeProvider>
    )
    expect(screen.getByTestId('theme-value').textContent).toBe('dark')
  })

  it('toggleTheme switches theme to light', () => {
    render(
      <ThemeProvider>
        <ThemeConsumer />
      </ThemeProvider>
    )

    act(() => {
      fireEvent.click(screen.getByText('Toggle'))
    })

    expect(screen.getByTestId('theme-value').textContent).toBe('light')
  })

  it('theme is persisted in localStorage under key tg-theme', () => {
    render(
      <ThemeProvider>
        <ThemeConsumer />
      </ThemeProvider>
    )

    expect(localStorage.getItem('tg-theme')).toBe('dark')

    act(() => {
      fireEvent.click(screen.getByText('Toggle'))
    })

    expect(localStorage.getItem('tg-theme')).toBe('light')
  })

  it('reads persisted theme from localStorage on mount', () => {
    localStorage.setItem('tg-theme', 'light')

    render(
      <ThemeProvider>
        <ThemeConsumer />
      </ThemeProvider>
    )

    expect(screen.getByTestId('theme-value').textContent).toBe('light')
  })
})
