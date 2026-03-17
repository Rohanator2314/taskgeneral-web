import { useState, useCallback } from 'react';

export interface TaskFilterParams {
  status?: string;
  project?: string;
  tag?: string;
  sort_by?: string;
}

const STORAGE_KEY = 'taskgeneral_preferences';

export interface UrgencyWeights {
  next: number;
  due: number;
  priorityHigh: number;
  active: number;
  priorityMedium: number;
  age: number;
  priorityLow: number;
  waiting: number;
  tags: number;
  project: number;
}

interface Preferences {
  defaultSort: string;
  defaultStatus: string;
  urgencyWeights: UrgencyWeights;
}

const defaultUrgencyWeights: UrgencyWeights = {
  next: 15,
  due: 12,
  priorityHigh: 6,
  active: 4,
  priorityMedium: 3.9,
  age: 2,
  priorityLow: 1.8,
  waiting: -3,
  tags: 1,
  project: 1,
};

const defaultPreferences: Preferences = {
  defaultSort: 'urgency',
  defaultStatus: 'pending',
  urgencyWeights: defaultUrgencyWeights,
};

function loadPreferences(): Preferences {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      const parsed = JSON.parse(stored);
      return {
        ...defaultPreferences,
        ...parsed,
        urgencyWeights: { ...defaultUrgencyWeights, ...parsed.urgencyWeights },
      };
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

  const calculateUrgency = useCallback((task: {
    is_active: boolean;
    is_waiting: boolean;
    tags: string[];
    project?: string;
    priority?: string;
    due?: string;
    entry?: string;
  }): number => {
    const w = prefs.urgencyWeights;
    let score = 0;

    if (task.tags.includes('next')) score += w.next;

    if (task.due) {
      const dueDate = new Date(task.due);
      const now = new Date();
      const days = (dueDate.getTime() - now.getTime()) / (1000 * 60 * 60 * 24);
      if (days < 0) score += w.due;
      else if (days < 7) score += w.due * (1 - days / 7);
    }

    const p = task.priority || '';
    if (p === 'H') score += w.priorityHigh;
    if (p === 'M') score += w.priorityMedium;
    if (p === 'L') score += w.priorityLow;

    if (task.is_active) score += w.active;
    if (task.is_waiting) score += w.waiting;

    if (task.entry) {
      const entryDate = new Date(task.entry);
      const now = new Date();
      const ageDays = (now.getTime() - entryDate.getTime()) / (1000 * 60 * 60 * 24);
      score += w.age * Math.min(ageDays / 365, 1);
    }

    if (task.tags.length > 0) {
      score += w.tags * (task.tags.length === 1 ? 0.8 : task.tags.length === 2 ? 0.9 : 1);
    }

    if (task.project) score += w.project;

    return score;
  }, [prefs.urgencyWeights]);

  const resetUrgencyWeights = useCallback(() => {
    updatePreferences({ urgencyWeights: defaultUrgencyWeights });
  }, [updatePreferences]);

  return { prefs, updatePreferences, getDefaultFilter, calculateUrgency, resetUrgencyWeights };
}
