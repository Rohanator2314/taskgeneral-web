import { useState, useCallback } from 'react';

export interface TaskFilterParams {
  status?: string;
  project?: string;
  tag?: string;
  sort_by?: string;
}

const STORAGE_KEY = 'taskgeneral_preferences';

interface Preferences {
  defaultSort: string;
  defaultStatus: string;
}

const defaultPreferences: Preferences = {
  defaultSort: 'urgency',
  defaultStatus: '',
};

function loadPreferences(): Preferences {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      return { ...defaultPreferences, ...JSON.parse(stored) };
    }
  } catch { }
  return defaultPreferences;
}

export function usePreferences() {
  const [prefs, setPrefs] = useState<Preferences>(() => loadPreferences());

  const updatePreferences = useCallback((updates: Partial<Preferences>) => {
    setPrefs(current => {
      const next = { ...current, ...updates };
      localStorage.setItem(STORAGE_KEY, JSON.stringify(next));
      return next;
    });
  }, []);

  const getDefaultFilter = useCallback((): TaskFilterParams => {
    return {
      sort_by: prefs.defaultSort,
      status: prefs.defaultStatus || undefined,
    };
  }, [prefs]);

  return { prefs, updatePreferences, getDefaultFilter };
}
