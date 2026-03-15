import { type ReactNode } from 'react'
import { useTheme } from '../theme/ThemeContext'

interface LayoutProps {
  children: ReactNode
}

export default function Layout({ children }: LayoutProps) {
  const { theme, toggleTheme } = useTheme()
  const isDark = theme === 'dark'

  return (
    <div className="min-h-screen bg-bg-primary text-text-primary font-mono p-4 flex flex-col items-center justify-center">
      <div className="w-full max-w-4xl border border-border rounded-lg shadow-lg overflow-hidden flex flex-col min-h-[600px]">
        <header 
          data-testid="layout-header" 
          className="border-b border-border p-3 flex justify-between items-center bg-bg-primary select-none"
        >
          <div className="flex items-center gap-2">
            <span className="text-accent font-bold">┌─</span>
            <span className="font-bold">TaskGeneral</span>
            <span className="hidden sm:inline text-border">──────────────────────</span>
          </div>
          
          <div className="flex items-center gap-4">
            <button 
              onClick={toggleTheme}
              data-testid="theme-toggle"
              className="hover:text-accent transition-colors cursor-pointer"
              title={isDark ? "Switch to Light Mode" : "Switch to Dark Mode"}
            >
              [{isDark ? '☀' : '🌙'}]
            </button>
            <div className="flex items-center gap-1 text-sm">
              <span>[Sync]</span>
              <span className="text-accent font-bold">─┐</span>
            </div>
          </div>
        </header>

        <main className="flex-1 p-6 relative overflow-y-auto">
           {children}
        </main>

        <footer className="border-t border-border p-2 text-sm flex items-center gap-2 bg-bg-primary select-none">
          <span className="text-accent font-bold">└─</span>
          <span className="text-green-500">●</span>
          <span>Connected</span>
          <span className="text-border">|</span>
          <span>Last sync: never</span>
          <span className="flex-1 text-border overflow-hidden whitespace-nowrap">
            ────────────────────────────────────────────────────────────────────────
          </span>
          <span className="text-accent font-bold">┘</span>
        </footer>
      </div>
    </div>
  )
}
