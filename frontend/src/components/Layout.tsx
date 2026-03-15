import { useState, useEffect } from 'react'
import { useTheme } from '../theme/ThemeContext'
import TaskList from './TaskList'
import SettingsPage from './SettingsPage'
import FilterBar from './FilterBar'
import type { TaskFilterParams } from '../api/types'

type View = 'tasks' | 'settings'

export default function Layout() {
  const { theme, toggleTheme } = useTheme()
  const isDark = theme === 'dark'
  const [view, setView] = useState<View>('tasks')

  const [filter, setFilter] = useState<TaskFilterParams>(() => {
    const params = new URLSearchParams(window.location.search);
    const f: TaskFilterParams = {};
    if (params.get('status')) f.status = params.get('status')!;
    if (params.get('project')) f.project = params.get('project')!;
    if (params.get('tag')) f.tag = params.get('tag')!;
    if (params.get('sort_by')) f.sort_by = params.get('sort_by')!;
    return f;
  });

  useEffect(() => {
    if (view !== 'settings') return;
    const handler = (e: KeyboardEvent) => {
      if (e.key === 'Escape') setView('tasks');
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [view]);

  const handleFilterChange = (newFilter: TaskFilterParams) => {
    const params = new URLSearchParams();
    if (newFilter.status) params.set('status', newFilter.status);
    if (newFilter.project) params.set('project', newFilter.project);
    if (newFilter.tag) params.set('tag', newFilter.tag);
    if (newFilter.sort_by) params.set('sort_by', newFilter.sort_by);
    
    const search = params.toString();
    const newUrl = search ? `?${search}` : window.location.pathname;
    window.history.pushState({}, '', newUrl);
    setFilter(newFilter);
  };

  return (
    <div className="h-screen bg-bg-primary text-text-primary font-mono p-4 flex flex-col items-center justify-center overflow-hidden">
      <div className="w-full max-w-4xl border border-border rounded-lg shadow-lg flex flex-col h-full">
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
              <button
                onClick={() => setView(v => v === 'settings' ? 'tasks' : 'settings')}
                data-testid="settings-nav"
                className={`hover:text-accent transition-colors cursor-pointer ${view === 'settings' ? 'text-accent' : ''}`}
                title="Settings"
              >
                {view === 'settings' ? '[← Tasks]' : '[Settings]'}
              </button>
              <span className="text-accent font-bold">─┐</span>
            </div>
          </div>
        </header>

        <main className="flex-1 relative overflow-hidden flex flex-col">
          {view === 'settings' ? (
            <SettingsPage onBack={() => setView('tasks')} />
          ) : (
            <>
              <FilterBar filter={filter} onFilterChange={handleFilterChange} />
              <TaskList filter={filter} />
            </>
          )}
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
