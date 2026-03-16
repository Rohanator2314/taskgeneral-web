import { describe, it, expect, vi } from 'vitest'
import { screen, waitFor, fireEvent } from '@testing-library/react'
import { render } from '../../test/utils'
import { server } from '../../test/mocks/server'
import { http, HttpResponse } from 'msw'
import SettingsPage from '../SettingsPage'

const defaultPrefs = {
  defaultSort: 'urgency',
  defaultStatus: '',
}

const mockUpdatePrefs = vi.fn()
const mockResetToDefaults = vi.fn()

describe('SettingsPage', () => {
  it('renders all section headings', () => {
    render(<SettingsPage onBack={vi.fn()} prefs={defaultPrefs} updatePreferences={mockUpdatePrefs} onResetToDefaults={mockResetToDefaults} />)

    expect(screen.getByText(/\[ Sync \]/i)).toBeInTheDocument()
    expect(screen.getByText(/\[ Appearance \]/i)).toBeInTheDocument()
    expect(screen.getByText(/\[ Danger Zone \]/i)).toBeInTheDocument()
  })

  it('calls onBack when back button clicked', () => {
    const onBack = vi.fn()
    render(<SettingsPage onBack={onBack} prefs={defaultPrefs} updatePreferences={mockUpdatePrefs} onResetToDefaults={mockResetToDefaults} />)

    fireEvent.click(screen.getByText(/← Back to Tasks/i))

    expect(onBack).toHaveBeenCalledOnce()
  })

  it('shows About section with version from API', async () => {
    render(<SettingsPage onBack={vi.fn()} prefs={defaultPrefs} updatePreferences={mockUpdatePrefs} onResetToDefaults={mockResetToDefaults} />)

    await waitFor(() => {
      expect(screen.getByText(/\[ About \]/i)).toBeInTheDocument()
      expect(screen.getByText(/Version: 1\.0\.0/i)).toBeInTheDocument()
    })
  })

  it('shows confirmation step when Clear All Data is clicked once', async () => {
    render(<SettingsPage onBack={vi.fn()} prefs={defaultPrefs} updatePreferences={mockUpdatePrefs} onResetToDefaults={mockResetToDefaults} />)

    fireEvent.click(screen.getByTestId('clear-data-btn'))

    expect(screen.getByText(/Are you sure\?/i)).toBeInTheDocument()
    expect(screen.getByText('Yes, delete all')).toBeInTheDocument()
    expect(screen.getByText('Cancel')).toBeInTheDocument()
    expect(screen.queryByTestId('clear-data-btn')).not.toBeInTheDocument()
  })

  it('cancels clear confirmation when Cancel is clicked', async () => {
    render(<SettingsPage onBack={vi.fn()} prefs={defaultPrefs} updatePreferences={mockUpdatePrefs} onResetToDefaults={mockResetToDefaults} />)

    fireEvent.click(screen.getByTestId('clear-data-btn'))
    expect(screen.getByText(/Are you sure\?/i)).toBeInTheDocument()

    fireEvent.click(screen.getByText('Cancel'))

    expect(screen.queryByText(/Are you sure\?/i)).not.toBeInTheDocument()
    expect(screen.getByTestId('clear-data-btn')).toBeInTheDocument()
  })

  it('shows success status after confirming clear data', async () => {
    render(<SettingsPage onBack={vi.fn()} prefs={defaultPrefs} updatePreferences={mockUpdatePrefs} onResetToDefaults={mockResetToDefaults} />)

    fireEvent.click(screen.getByTestId('clear-data-btn'))
    fireEvent.click(screen.getByText('Yes, delete all'))

    await waitFor(() => {
      expect(screen.getByTestId('settings-status')).toBeInTheDocument()
      expect(screen.getByText(/All data cleared\./i)).toBeInTheDocument()
    })
  })

  it('shows success status after saving sync config', async () => {
    render(<SettingsPage onBack={vi.fn()} prefs={defaultPrefs} updatePreferences={mockUpdatePrefs} onResetToDefaults={mockResetToDefaults} />)

    fireEvent.change(screen.getByPlaceholderText('https://sync.example.com'), {
      target: { value: 'https://sync.test.com' },
    })
    fireEvent.change(screen.getByPlaceholderText('desktop-main'), {
      target: { value: 'my-client' },
    })
    fireEvent.click(screen.getByRole('button', { name: /^Save$/i }))

    await waitFor(() => {
      expect(screen.getByTestId('settings-status')).toBeInTheDocument()
      expect(screen.getByText(/Configuration saved\./i)).toBeInTheDocument()
    })
  })

  it('shows error status when save config fails', async () => {
    server.use(
      http.post('/api/sync/configure', () =>
        HttpResponse.json({ error: 'Server unreachable' }, { status: 500 })
      )
    )

    render(<SettingsPage onBack={vi.fn()} prefs={defaultPrefs} updatePreferences={mockUpdatePrefs} onResetToDefaults={mockResetToDefaults} />)

    fireEvent.click(screen.getByRole('button', { name: /^Save$/i }))

    await waitFor(() => {
      expect(screen.getByTestId('settings-status')).toBeInTheDocument()
      expect(screen.getByText(/Failed to save configuration\./i)).toBeInTheDocument()
    })
  })

  it('shows error status when Sync Now fails (409)', async () => {
    render(<SettingsPage onBack={vi.fn()} prefs={defaultPrefs} updatePreferences={mockUpdatePrefs} onResetToDefaults={mockResetToDefaults} />)

    fireEvent.click(screen.getByRole('button', { name: /Sync Now/i }))

    await waitFor(() => {
      expect(screen.getByTestId('settings-status')).toBeInTheDocument()
      expect(screen.getByText(/Sync not configured/i)).toBeInTheDocument()
    })
  })

  it('clears status banner when a new action starts', async () => {
    render(<SettingsPage onBack={vi.fn()} prefs={defaultPrefs} updatePreferences={mockUpdatePrefs} onResetToDefaults={mockResetToDefaults} />)

    fireEvent.click(screen.getByRole('button', { name: /Sync Now/i }))
    await waitFor(() => {
      expect(screen.getByTestId('settings-status')).toBeInTheDocument()
    })

    fireEvent.click(screen.getByRole('button', { name: /^Save$/i }))
    await waitFor(() => {
      expect(screen.getByText(/Configuration saved\./i)).toBeInTheDocument()
    })
  })

  it('theme toggle button reflects current theme', () => {
    render(<SettingsPage onBack={vi.fn()} prefs={defaultPrefs} updatePreferences={mockUpdatePrefs} onResetToDefaults={mockResetToDefaults} />)
    const themeBtn = screen.getByRole('button', { name: /Light|Dark/ })
    expect(themeBtn).toBeInTheDocument()
  })
})
