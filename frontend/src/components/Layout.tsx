import { useState, useEffect } from 'react'
import { useTheme } from '../theme/ThemeContext'
import { usePreferences } from '../hooks/usePreferences'
import { useAuth } from '../auth/AuthContext'
import { useSyncConfigStatus, useSync } from '../api/hooks'
import TaskList from './TaskList'
import SettingsPage from './SettingsPage'
import UrgencyWeightsPage from './UrgencyWeightsPage'
import FilterBar from './FilterBar'
import type { TaskFilterParams } from '../api/types'

type View = 'tasks' | 'settings' | 'urgency-weights'

interface LastSyncInfo {
  timestamp: number;
  message: string;
}

const LAST_SYNC_KEY = 'taskgeneral_last_sync';

function formatLastSync(info: LastSyncInfo | null): string {
  if (!info) return 'Last sync: never';
  const mins = Math.floor((Date.now() - info.timestamp) / 60000);
  if (mins < 1) return `Last sync: just now - ${info.message}`;
  if (mins < 60) return `Last sync: ${mins}min ago - ${info.message}`;
  const hours = Math.floor(mins / 60);
  if (hours < 24) return `Last sync: ${hours}h ago - ${info.message}`;
  const days = Math.floor(hours / 24);
  return `Last sync: ${days}d ago - ${info.message}`;
}

export default function Layout() {
  const { theme, toggleTheme } = useTheme()
  const { signOut } = useAuth()
  const isDark = theme === 'dark'
  const [view, setView] = useState<View>('tasks')
  const { prefs, updatePreferences, getDefaultFilter, calculateUrgency, resetUrgencyWeights } = usePreferences()
  const { data: syncStatus } = useSyncConfigStatus()
  const syncMutation = useSync()
  const [isSyncing, setIsSyncing] = useState(false)
  const [lastSync, setLastSync] = useState<LastSyncInfo | null>(() => {
    const stored = localStorage.getItem(LAST_SYNC_KEY);
    return stored ? JSON.parse(stored) : null;
  })

  const [filter, setFilter] = useState<TaskFilterParams>(() => {
    const params = new URLSearchParams(window.location.search);
    if (params.toString()) {
      const f: TaskFilterParams = { sort_by: 'urgency' };
      if (params.get('status')) f.status = params.get('status')!;
      if (params.get('project')) f.project = params.get('project')!;
      if (params.get('tag')) f.tag = params.get('tag')!;
      if (params.get('sort_by')) f.sort_by = params.get('sort_by')!;
      return f;
    }
    return getDefaultFilter();
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
    params.set('sort_by', newFilter.sort_by || 'urgency');
    
    const search = params.toString();
    const newUrl = search ? `?${search}` : window.location.pathname;
    window.history.pushState({}, '', newUrl);
    setFilter(newFilter);
  };

  const handleResetToDefaults = () => {
    setFilter(getDefaultFilter());
    const params = new URLSearchParams();
    const f = getDefaultFilter();
    if (f.status) params.set('status', f.status);
    if (f.project) params.set('project', f.project);
    if (f.tag) params.set('tag', f.tag);
    params.set('sort_by', f.sort_by || 'urgency');
    window.history.pushState({}, '', params.toString() ? `?${params.toString()}` : window.location.pathname);
  };

  const handleSync = () => {
    if (isSyncing || !syncStatus?.configured) return;
    setIsSyncing(true);
    syncMutation.mutate(undefined, {
      onSuccess: (data) => {
        const info: LastSyncInfo = { timestamp: Date.now(), message: data.message };
        setLastSync(info);
        localStorage.setItem(LAST_SYNC_KEY, JSON.stringify(info));
      },
      onSettled: () => setIsSyncing(false),
    });
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
            <button 
              onClick={() => signOut()}
              className="hover:text-accent transition-colors cursor-pointer"
              title="Sign Out"
            >
              [Logout]
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
          {view === 'urgency-weights' ? (
            <UrgencyWeightsPage
              weights={prefs.urgencyWeights}
              onSave={(weights) => updatePreferences({ urgencyWeights: weights })}
              onReset={resetUrgencyWeights}
              onBack={() => setView('settings')}
            />
          ) : view === 'settings' ? (
            <SettingsPage 
              onBack={() => setView('tasks')} 
              prefs={prefs}
              updatePreferences={updatePreferences}
              onResetToDefaults={handleResetToDefaults}
              onOpenUrgencyWeights={() => setView('urgency-weights')}
            />
          ) : (
            <>
              <FilterBar filter={filter} onFilterChange={handleFilterChange} />
              <TaskList filter={filter} onSync={handleSync} calculateUrgency={calculateUrgency} />
            </>
          )}
        </main>

        <footer className="border-t border-border p-2 text-sm flex items-center gap-2 bg-bg-primary select-none">
          <span className="text-accent font-bold">└─</span>
          {isSyncing ? (
            <>
              <span className="text-green-500 animate-pulse">●</span>
              <span>Syncing...</span>
            </>
          ) : syncStatus?.configured ? (
            <>
              <span className="text-green-500">●</span>
              <span>Sync configured</span>
            </>
          ) : (
            <>
              <span className="text-yellow-500">●</span>
              <span>Sync not configured</span>
            </>
          )}
          <span className="text-border">|</span>
          <span>{formatLastSync(lastSync)}</span>
          <span className="flex-1 text-border overflow-hidden whitespace-nowrap">
            ────────────────────────────────────────────────────────────────────────
          </span>
          <span className="text-accent font-bold">┘</span>
        </footer>
      </div>
    </div>
  )
}
