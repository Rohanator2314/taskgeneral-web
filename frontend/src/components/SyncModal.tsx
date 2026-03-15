import React, { useState, useEffect } from 'react';
import { useSyncConfig, useSync, useClearData } from '../api/hooks';

interface SyncModalProps {
  onClose: () => void;
}

export default function SyncModal({ onClose }: SyncModalProps) {
  const [serverUrl, setServerUrl] = useState('');
  const [encryptionSecret, setEncryptionSecret] = useState('');
  const [clientId, setClientId] = useState('');
  const [statusMessage, setStatusMessage] = useState<string | null>(null);
  const [isError, setIsError] = useState(false);

  const syncConfigMutation = useSyncConfig();
  const syncMutation = useSync();
  const clearDataMutation = useClearData();

  const isLoading = syncConfigMutation.isPending || syncMutation.isPending || clearDataMutation.isPending;

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [onClose]);

  const handleSaveConfig = async (e: React.FormEvent) => {
    e.preventDefault();
    setStatusMessage('Saving configuration...');
    setIsError(false);
    
    try {
      await syncConfigMutation.mutateAsync({
        server_url: serverUrl,
        encryption_secret: encryptionSecret,
        client_id: clientId,
      });
      setStatusMessage('Configuration saved.');
    } catch (err) {
      setIsError(true);
      setStatusMessage('Failed to save configuration.');
    }
  };

  const handleSyncNow = async () => {
    setStatusMessage('Syncing...');
    setIsError(false);
    try {
      const result = await syncMutation.mutateAsync();
      if (result.success) {
        setStatusMessage(result.message);
      } else {
        setIsError(true);
        setStatusMessage(result.message);
      }
    } catch (err: any) {
      setIsError(true);
      setStatusMessage(err.message || 'Sync failed');
    }
  };

  const handleClearData = async () => {
    if (window.confirm("This will delete all local task data. Are you sure?")) {
      setStatusMessage('Clearing data...');
      setIsError(false);
      try {
        await clearDataMutation.mutateAsync();
        onClose();
      } catch (err) {
        setIsError(true);
        setStatusMessage('Failed to clear data.');
      }
    }
  };

  return (
    <div 
      className="fixed inset-0 bg-black/60 z-50 flex items-center justify-center p-4 backdrop-blur-sm"
      onClick={onClose}
      data-testid="sync-modal-backdrop"
    >
      <div 
        className="bg-bg-primary border border-border text-text-primary font-mono w-full max-w-md shadow-2xl relative flex flex-col max-h-[90vh] overflow-y-auto"
        data-testid="sync-modal"
        onClick={e => e.stopPropagation()}
      >
        <div className="flex justify-between items-center border-b border-border p-4 bg-bg-primary sticky top-0 z-10">
          <h2 className="text-lg font-bold text-accent">[ Sync Configuration ]</h2>
          <button 
            onClick={onClose}
            className="text-text-primary hover:text-accent transition-colors"
            aria-label="Close"
          >
            [x]
          </button>
        </div>

        <div className="p-6 space-y-6">
          <form onSubmit={handleSaveConfig} className="space-y-4">
            <div className="space-y-1">
              <label className="block text-sm opacity-80">Server URL</label>
              <input
                type="text"
                value={serverUrl}
                onChange={e => setServerUrl(e.target.value)}
                className="w-full bg-bg-primary border border-border p-2 focus:border-accent focus:outline-none transition-colors"
                placeholder="https://api.taskgeneral.com"
                disabled={isLoading}
              />
            </div>
            
            <div className="space-y-1">
              <label className="block text-sm opacity-80">Client ID</label>
               <input
                type="text"
                value={clientId}
                onChange={e => setClientId(e.target.value)}
                className="w-full bg-bg-primary border border-border p-2 focus:border-accent focus:outline-none transition-colors"
                placeholder="desktop-main"
                disabled={isLoading}
              />
            </div>

            <div className="space-y-1">
              <label className="block text-sm opacity-80">Encryption Secret</label>
              <input
                type="password"
                value={encryptionSecret}
                onChange={e => setEncryptionSecret(e.target.value)}
                className="w-full bg-bg-primary border border-border p-2 focus:border-accent focus:outline-none transition-colors"
                placeholder="••••••••"
                disabled={isLoading}
              />
            </div>

            <button
              type="submit"
              disabled={isLoading}
              className="w-full border border-accent text-accent hover:bg-accent hover:text-bg-primary py-2 px-4 transition-all disabled:opacity-50 disabled:cursor-not-allowed font-bold"
            >
              {syncConfigMutation.isPending ? 'Saving...' : 'Save Configuration'}
            </button>
          </form>

          <div className="border-t border-border border-dashed pt-6">
            <button
              onClick={handleSyncNow}
              disabled={isLoading}
              className="w-full border border-text-primary text-text-primary hover:border-accent hover:text-accent py-2 px-4 transition-all disabled:opacity-50 disabled:cursor-not-allowed flex justify-center items-center gap-2"
            >
              {syncMutation.isPending ? (
                <>
                  <span className="animate-spin">⟳</span> Syncing...
                </>
              ) : (
                'Sync Now'
              )}
            </button>
            
            {statusMessage && (
              <div className={`mt-4 text-sm p-3 border ${isError ? 'border-red-500 text-red-500 bg-red-500/10' : 'border-green-500 text-green-500 bg-green-500/10'}`}>
                <span className="font-bold mr-2">
                  {isError ? '[ ERROR ]' : '[ SUCCESS ]'}
                </span>
                {statusMessage}
              </div>
            )}
          </div>

          <div className="border-t border-border border-dashed pt-6 mt-6">
            <div className="bg-red-500/5 border border-red-500/30 p-4">
              <h3 className="text-red-500 text-sm font-bold mb-2 uppercase tracking-wider">Danger Zone</h3>
              <p className="text-xs text-text-primary/70 mb-3">
                This will delete all local tasks and data. This action cannot be reversed.
              </p>
              <button
                onClick={handleClearData}
                disabled={isLoading}
                className="w-full border border-red-500 text-red-500 hover:bg-red-500 hover:text-white py-2 px-4 transition-colors disabled:opacity-50 text-sm"
              >
                Clear Local Data
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
