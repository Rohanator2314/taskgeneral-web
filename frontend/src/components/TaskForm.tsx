import { useState, useEffect } from 'react';
import { useCreateTask, useUpdateTask } from '../api/hooks';
import type { TaskInfo, TaskUpdateParams } from '../api/types';

interface TaskFormProps {
  mode: 'create' | 'edit';
  task?: TaskInfo;
  onClose: () => void;
}

export default function TaskForm({ mode, task, onClose }: TaskFormProps) {
  const [description, setDescription] = useState(task?.description || '');
  const [project, setProject] = useState(task?.project || '');
  const [priority, setPriority] = useState(task?.priority || '');
  const [due, setDue] = useState(task?.due || '');
  const [tags, setTags] = useState(task?.tags?.join(' ') || '');
  const [statusMsg, setStatusMsg] = useState<string | null>(null);

  const createTask = useCreateTask();
  const updateTask = useUpdateTask();

  useEffect(() => {
    if (statusMsg) {
      const timer = setTimeout(() => setStatusMsg(null), 2000);
      return () => clearTimeout(timer);
    }
  }, [statusMsg]);

  const inputClass = "border border-border bg-transparent text-text-primary font-mono px-1 outline-none focus:border-accent";

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    if (mode === 'create') {
      if (!description.trim()) return;
      createTask.mutate(description, {
        onSuccess: () => {
          setStatusMsg('Saved');
          setDescription('');
          onClose(); 
        },
        onError: () => setStatusMsg('Error')
      });
    } else if (mode === 'edit' && task) {
      const updates: TaskUpdateParams = {};
      if (description !== task.description) updates.description = description;
      if (project !== (task.project || '')) updates.project = project;
      if (priority !== (task.priority || '')) updates.priority = priority;
      if (due !== (task.due || '')) updates.due = due;
      
      const currentTags = task.tags || [];
      const newTags = tags.split(' ').filter(t => t.trim());
      if (JSON.stringify(currentTags) !== JSON.stringify(newTags)) {
        updates.tags = newTags;
      }

      if (Object.keys(updates).length === 0) {
        onClose();
        return;
      }

      updateTask.mutate({ uuid: task.uuid, updates }, {
        onSuccess: () => {
          setStatusMsg('Saved');
          onClose();
        },
        onError: () => setStatusMsg('Error')
      });
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      onClose();
    }
  };

  if (mode === 'create') {
    return (
      <div className="p-2 border-b border-border bg-bg-primary">
        <form onSubmit={handleSubmit} className="flex gap-2 items-center">
          <span className="text-accent font-mono">➜</span>
          <input
            autoFocus
            type="text"
            className={`${inputClass} w-full`}
            placeholder="Add new task..."
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            onKeyDown={handleKeyDown}
          />
          {statusMsg && <span className="text-green-500 font-mono text-xs">{statusMsg}</span>}
        </form>
      </div>
    );
  }

  return (
    <tr className="bg-bg-primary">
      <td colSpan={8} className="p-2 border border-accent">
        <form onSubmit={handleSubmit} className="flex flex-col gap-2">
          <div className="flex gap-2">
            <input
              autoFocus
              type="text"
              className={`${inputClass} flex-grow`}
              placeholder="Description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              onKeyDown={handleKeyDown}
            />
          </div>
          <div className="flex gap-2 text-sm">
            <input
              type="text"
              className={`${inputClass} w-32`}
              placeholder="Project"
              value={project}
              onChange={(e) => setProject(e.target.value)}
              onKeyDown={handleKeyDown}
            />
            <input
              type="text"
              className={`${inputClass} w-48`}
              placeholder="Tags (space sep)"
              value={tags}
              onChange={(e) => setTags(e.target.value)}
              onKeyDown={handleKeyDown}
            />
            <select
              className={`${inputClass} w-20`}
              value={priority}
              onChange={(e) => setPriority(e.target.value)}
              onKeyDown={handleKeyDown}
            >
              <option value="">Pri</option>
              <option value="H">H</option>
              <option value="M">M</option>
              <option value="L">L</option>
            </select>
            <div className="flex items-center gap-1">
              <input
                type="date"
                className={`${inputClass} w-32`}
                placeholder="Due"
                value={due}
                onChange={(e) => setDue(e.target.value)}
                onKeyDown={handleKeyDown}
              />
              {due && (
                <button
                  type="button"
                  onClick={() => setDue('')}
                  className="text-xs text-red-500 hover:text-red-400"
                  title="Clear due date"
                >
                  ×
                </button>
              )}
            </div>
            <div className="flex-grow text-right">
              {statusMsg && <span className="text-green-500 font-mono text-xs mr-2">{statusMsg}</span>}
              <button type="button" onClick={onClose} className="mr-2 text-text-primary hover:text-accent">Cancel</button>
              <button type="submit" className="text-accent hover:text-white border border-accent px-2">Save</button>
            </div>
          </div>
        </form>
      </td>
    </tr>
  );
}
