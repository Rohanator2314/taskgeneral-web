import { useState } from 'react';
import { useSyncConfig, useSync, useClearData, useVersion } from '../api/hooks';
import { useTheme } from '../theme/ThemeContext';

interface Preferences {
  defaultSort: string;
  defaultStatus: string;
}

interface SettingsPageProps {
  onBack: () => void;
  prefs: Preferences;
  updatePreferences: (updates: Partial<Preferences>) => void;
  onResetToDefaults: () => void;
}

export default function SettingsPage({ onBack, prefs, updatePreferences, onResetToDefaults }: SettingsPageProps) {
  const [serverUrl, setServerUrl] = useState('');
  const [clientId, setClientId] = useState('');
  const [encryptionSecret, setEncryptionSecret] = useState('');
  const [statusMsg, setStatusMsg] = useState<string | null>(null);
  const [isStatusError, setIsStatusError] = useState(false);
  const [pendingClear, setPendingClear] = useState(false);

  const { theme, toggleTheme } = useTheme();
  const { data: version } = useVersion();

  const syncConfigMutation = useSyncConfig();
  const syncMutation = useSync();
  const clearDataMutation = useClearData();

  const isBusy = syncConfigMutation.isPending || syncMutation.isPending || clearDataMutation.isPending;

  const showStatus = (msg: string, error = false) => {
    setStatusMsg(msg);
    setIsStatusError(error);
  };

  const handleSaveConfig = async (e: { preventDefault(): void }) => {
    e.preventDefault();
    setStatusMsg(null);
    try {
      await syncConfigMutation.mutateAsync({
        server_url: serverUrl,
        encryption_secret: encryptionSecret,
        client_id: clientId,
      });
      showStatus('Configuration saved.');
    } catch {
      showStatus('Failed to save configuration.', true);
    }
  };

  const handleSyncNow = async () => {
    setStatusMsg(null);
    try {
      const result = await syncMutation.mutateAsync();
      showStatus(result.message, !result.success);
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : 'Sync failed.';
      showStatus(msg, true);
    }
  };

  const handleClearData = async () => {
    if (!pendingClear) {
      setPendingClear(true);
      return;
    }
    setPendingClear(false);
    try {
      await clearDataMutation.mutateAsync();
      showStatus('All data cleared.');
    } catch {
      showStatus('Failed to clear data.', true);
    }
  };

  return (
    <div className="flex-1 min-h-0 overflow-auto font-mono text-text-primary">
      <div className="max-w-xl mx-auto p-6 space-y-8">

        <section>
          <h2 className="text-accent font-bold text-xs uppercase tracking-widest mb-4">[ Sync ]</h2>
          <form onSubmit={handleSaveConfig} className="space-y-3">
            <div>
              <label className="block text-xs opacity-60 mb-1">Server URL</label>
              <input
                type="text"
                value={serverUrl}
                onChange={e => setServerUrl(e.target.value)}
                placeholder="https://sync.example.com"
                disabled={isBusy}
                className="w-full bg-bg-primary border border-border px-2 py-1.5 text-sm focus:border-accent focus:outline-none transition-colors disabled:opacity-50"
              />
            </div>
            <div>
              <label className="block text-xs opacity-60 mb-1">Client ID</label>
              <input
                type="text"
                value={clientId}
                onChange={e => setClientId(e.target.value)}
                placeholder="desktop-main"
                disabled={isBusy}
                className="w-full bg-bg-primary border border-border px-2 py-1.5 text-sm focus:border-accent focus:outline-none transition-colors disabled:opacity-50"
              />
            </div>
            <div>
              <label className="block text-xs opacity-60 mb-1">Encryption Secret</label>
              <input
                type="password"
                value={encryptionSecret}
                onChange={e => setEncryptionSecret(e.target.value)}
                placeholder="••••••••"
                disabled={isBusy}
                className="w-full bg-bg-primary border border-border px-2 py-1.5 text-sm focus:border-accent focus:outline-none transition-colors disabled:opacity-50"
              />
            </div>
            <div className="flex gap-2 pt-1">
              <button
                type="submit"
                disabled={isBusy}
                className="border border-accent text-accent hover:bg-accent hover:text-bg-primary px-4 py-1 text-sm transition-all disabled:opacity-50"
              >
                {syncConfigMutation.isPending ? 'Saving...' : 'Save'}
              </button>
              <button
                type="button"
                onClick={handleSyncNow}
                disabled={isBusy}
                className="border border-border hover:border-accent px-4 py-1 text-sm transition-all disabled:opacity-50 flex items-center gap-1.5"
              >
                <span className={syncMutation.isPending ? 'animate-spin inline-block' : ''}>⟳</span>
                {syncMutation.isPending ? 'Syncing...' : 'Sync Now'}
              </button>
            </div>
          </form>
        </section>

        {statusMsg && (
          <div
            data-testid="settings-status"
            className={`text-sm px-3 py-2 border ${isStatusError ? 'border-red-500 text-red-500 bg-red-500/10' : 'border-green-500 text-green-500 bg-green-500/10'}`}
          >
            <span className="font-bold mr-2">{isStatusError ? '[ ERR ]' : '[ OK ]'}</span>
            {statusMsg}
          </div>
        )}

        <section className="border-t border-border pt-6">
          <h2 className="text-accent font-bold text-xs uppercase tracking-widest mb-4">[ Appearance ]</h2>
          <div className="flex items-center justify-between text-sm">
            <span>Theme</span>
            <button
              onClick={toggleTheme}
              className="border border-border px-3 py-1 hover:border-accent transition-colors"
            >
              {theme === 'dark' ? '☀ Light' : '🌙 Dark'}
            </button>
          </div>
        </section>

        <section className="border-t border-border pt-6">
          <h2 className="text-accent font-bold text-xs uppercase tracking-widest mb-4">[ Defaults ]</h2>
          <div className="space-y-3 text-sm">
            <div className="flex items-center justify-between">
              <span>Default Sort</span>
              <select
                value={prefs.defaultSort}
                onChange={(e) => updatePreferences({ defaultSort: e.target.value })}
                className="bg-bg-primary border border-border px-2 py-1 focus:border-accent focus:outline-none"
              >
                <option value="urgency">Urgency</option>
                <option value="due">Due Date</option>
                <option value="priority">Priority</option>
                <option value="entry">Entry Date</option>
                <option value="modified">Modified</option>
                <option value="description">Description</option>
              </select>
            </div>
            <div className="flex items-center justify-between">
              <span>Default Filter</span>
              <select
                value={prefs.defaultStatus}
                onChange={(e) => updatePreferences({ defaultStatus: e.target.value })}
                className="bg-bg-primary border border-border px-2 py-1 focus:border-accent focus:outline-none"
              >
                <option value="">[Status: All]</option>
                <option value="pending">Pending</option>
                <option value="completed">Completed</option>
                <option value="waiting">Waiting</option>
              </select>
            </div>
            <button
              onClick={onResetToDefaults}
              className="text-xs border border-border px-2 py-1 hover:border-accent transition-colors"
            >
              Reset View to Defaults
            </button>
          </div>
        </section>

        {version && (
          <section className="border-t border-border pt-6">
            <h2 className="text-accent font-bold text-xs uppercase tracking-widest mb-4">[ About ]</h2>
            <div className="text-sm opacity-60">
              <span>Version: {version}</span>
            </div>
          </section>
        )}

        <section className="border-t border-border pt-6">
          <h2 className="text-red-500 font-bold text-xs uppercase tracking-widest mb-4">[ Danger Zone ]</h2>
          <div className="bg-red-500/5 border border-red-500/30 p-4 space-y-3">
            <p className="text-xs opacity-60">
              Permanently deletes all local task data. This cannot be undone.
            </p>
            {pendingClear ? (
              <div className="flex items-center gap-2">
                <span className="text-red-400 text-sm">Are you sure?</span>
                <button
                  onClick={handleClearData}
                  disabled={isBusy}
                  className="border border-red-500 text-red-500 hover:bg-red-500 hover:text-white px-3 py-1 text-sm transition-colors disabled:opacity-50"
                >
                  {clearDataMutation.isPending ? 'Clearing...' : 'Yes, delete all'}
                </button>
                <button
                  onClick={() => setPendingClear(false)}
                  disabled={isBusy}
                  className="border border-border px-3 py-1 text-sm hover:border-accent transition-colors"
                >
                  Cancel
                </button>
              </div>
            ) : (
              <button
                onClick={handleClearData}
                disabled={isBusy}
                data-testid="clear-data-btn"
                className="border border-red-500 text-red-500 hover:bg-red-500 hover:text-white px-3 py-1 text-sm transition-colors disabled:opacity-50"
              >
                Clear All Data
              </button>
            )}
          </div>
        </section>

        <div className="border-t border-border pt-4">
          <button
            onClick={onBack}
            className="text-sm opacity-60 hover:opacity-100 hover:text-accent transition-all"
          >
            ← Back to Tasks
          </button>
        </div>

      </div>
    </div>
  );
}
