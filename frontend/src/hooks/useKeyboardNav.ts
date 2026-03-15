import { useEffect, useCallback, useRef } from 'react';
import type { TaskInfo } from '../api/types';

interface UseKeyboardNavOptions {
  tasks: TaskInfo[] | undefined;
  selectedRow: number | null;
  setSelectedRow: (i: number | null) => void;
  formMode: 'create' | 'edit' | null;
  setFormMode: (m: 'create' | 'edit' | null) => void;
  pendingDelete: string | null;
  setPendingDelete: (uuid: string | null) => void;
  onComplete: (uuid: string) => void;
  onUncomplete: (uuid: string) => void;
  onStart: (uuid: string) => void;
  onStop: (uuid: string) => void;
  onDelete: (uuid: string) => void;
}

/**
 * Custom hook for keyboard navigation in the task list.
 *
 * Keyboard shortcuts:
 * - j/↓: Move to next task
 * - k/↑: Move to previous task
 * - gg: Jump to first task (vim-style double-g)
 * - G (shift+g): Jump to last task
 * - Enter/e: Edit selected task
 * - a: Create new task
 * - c: Complete selected task
 * - u: Uncomplete selected task
 * - s: Start/stop selected task (toggle)
 * - d: First press stages delete (shows confirmation); second press confirms
 * - Escape: Clear selection/cancel mode
 *
 * Guards:
 * - Keys disabled when input/select/textarea is focused
 * - Keys disabled when Ctrl/Meta modifier is held (preserves browser shortcuts)
 */
export function useKeyboardNav(options: UseKeyboardNavOptions): void {
  const {
    tasks,
    selectedRow,
    setSelectedRow,
    formMode,
    setFormMode,
    pendingDelete,
    setPendingDelete,
    onComplete,
    onUncomplete,
    onStart,
    onStop,
    onDelete,
  } = options;

  const lastKeyRef = useRef<string | null>(null);
  const lastKeyTimeRef = useRef<number>(0);

  const handleKeyDown = useCallback((e: KeyboardEvent) => {
    // Guard: never intercept when form inputs are focused
    if (
      document.activeElement instanceof HTMLInputElement ||
      document.activeElement instanceof HTMLTextAreaElement ||
      document.activeElement instanceof HTMLSelectElement
    ) {
      return;
    }

    // Guard: never intercept browser shortcuts (Ctrl+* or Cmd+*)
    if (e.ctrlKey || e.metaKey) {
      return;
    }

    const now = Date.now();
    const timeSinceLastKey = now - lastKeyTimeRef.current;

    if (e.key === 'j' || e.key === 'ArrowDown') {
      e.preventDefault();
      setPendingDelete(null);
      if (!tasks || tasks.length === 0) return;
      if (selectedRow === null) {
        setSelectedRow(0);
      } else if (selectedRow < tasks.length - 1) {
        setSelectedRow(selectedRow + 1);
      }
      lastKeyRef.current = e.key;
      lastKeyTimeRef.current = now;
      return;
    }

    if (e.key === 'k' || e.key === 'ArrowUp') {
      e.preventDefault();
      setPendingDelete(null);
      if (!tasks || tasks.length === 0) return;
      if (selectedRow === null) {
        setSelectedRow(tasks.length - 1);
      } else if (selectedRow > 0) {
        setSelectedRow(selectedRow - 1);
      }
      lastKeyRef.current = e.key;
      lastKeyTimeRef.current = now;
      return;
    }

    // vim-style double-g: track last 'g' press within 500ms window
    if (e.key === 'g') {
      if (lastKeyRef.current === 'g' && timeSinceLastKey < 500) {
        e.preventDefault();
        setPendingDelete(null);
        if (tasks && tasks.length > 0) {
          setSelectedRow(0);
        }
        lastKeyRef.current = null;
        lastKeyTimeRef.current = now;
      } else {
        lastKeyRef.current = 'g';
        lastKeyTimeRef.current = now;
      }
      return;
    }

    if (e.key === 'G') {
      e.preventDefault();
      setPendingDelete(null);
      if (tasks && tasks.length > 0) {
        setSelectedRow(tasks.length - 1);
      }
      lastKeyRef.current = e.key;
      lastKeyTimeRef.current = now;
      return;
    }

    if (e.key === 'a') {
      e.preventDefault();
      setPendingDelete(null);
      setFormMode('create');
      setSelectedRow(null);
      lastKeyRef.current = e.key;
      lastKeyTimeRef.current = now;
      return;
    }

    if (e.key === '/') {
      e.preventDefault();
      const projectInput = document.querySelector<HTMLInputElement>('[data-filter="project"]');
      projectInput?.focus();
      lastKeyRef.current = e.key;
      lastKeyTimeRef.current = now;
      return;
    }

    if (e.key === 'Escape') {
      setPendingDelete(null);
      setFormMode(null);
      setSelectedRow(null);
      lastKeyRef.current = e.key;
      lastKeyTimeRef.current = now;
      return;
    }

    if (selectedRow !== null && tasks && tasks[selectedRow]) {
      const task = tasks[selectedRow];

      if (e.key === 'Enter' || e.key === 'e') {
        e.preventDefault();
        setPendingDelete(null);
        setFormMode('edit');
        lastKeyRef.current = e.key;
        lastKeyTimeRef.current = now;
        return;
      }

      if (e.key === 'c') {
        e.preventDefault();
        setPendingDelete(null);
        if (task.status !== 'completed') {
          onComplete(task.uuid);
        }
        lastKeyRef.current = e.key;
        lastKeyTimeRef.current = now;
        return;
      }

      if (e.key === 'u') {
        e.preventDefault();
        setPendingDelete(null);
        if (task.status === 'completed') {
          onUncomplete(task.uuid);
        }
        lastKeyRef.current = e.key;
        lastKeyTimeRef.current = now;
        return;
      }

      if (e.key === 's') {
        e.preventDefault();
        setPendingDelete(null);
        if (task.is_active) {
          onStop(task.uuid);
        } else {
          onStart(task.uuid);
        }
        lastKeyRef.current = e.key;
        lastKeyTimeRef.current = now;
        return;
      }

      if (e.key === 'd') {
        e.preventDefault();
        if (pendingDelete === task.uuid) {
          onDelete(task.uuid);
          setPendingDelete(null);
          if (selectedRow < tasks.length - 1) {
            setSelectedRow(selectedRow);
          } else if (selectedRow > 0) {
            setSelectedRow(selectedRow - 1);
          } else {
            setSelectedRow(null);
          }
        } else {
          setPendingDelete(task.uuid);
        }
        lastKeyRef.current = e.key;
        lastKeyTimeRef.current = now;
        return;
      }
    }

    if (e.key !== 'g') {
      lastKeyRef.current = null;
    }
  }, [
    tasks,
    selectedRow,
    setSelectedRow,
    formMode,
    setFormMode,
    pendingDelete,
    setPendingDelete,
    onComplete,
    onUncomplete,
    onStart,
    onStop,
    onDelete,
  ]);

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);
}
