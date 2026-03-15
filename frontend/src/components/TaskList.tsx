import { useState, useEffect } from 'react';
import { 
  useTaskList, 
  useDeleteTask, 
  useCompleteTask, 
  useUncompleteTask, 
  useStartTask, 
  useStopTask 
} from '../api/hooks';
import type { TaskInfo } from '../api/types';
import TaskForm from './TaskForm';

const formatDuration = (dateStr?: string): string => {
  if (!dateStr) return '';
  const date = new Date(dateStr);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffDays = Math.floor(Math.abs(diffMs) / (1000 * 60 * 60 * 24));
  
  if (diffDays === 0) return '0d';
  if (diffDays < 7) return `${diffDays}d`;
  if (diffDays < 30) return `${Math.floor(diffDays / 7)}w`;
  return `${Math.floor(diffDays / 30)}mo`;
};

const formatDue = (dateStr?: string): string => {
  if (!dateStr) return '';
  const date = new Date(dateStr);
  const now = new Date();
  const diffMs = date.getTime() - now.getTime();
  const diffDays = Math.ceil(diffMs / (1000 * 60 * 60 * 24));

  if (diffDays < 0) return `${diffDays}d`;
  if (diffDays === 0) return '0d';
  if (diffDays < 7) return `${diffDays}d`;
  if (diffDays < 30) return `${Math.floor(diffDays / 7)}w`;
  return `${Math.floor(diffDays / 30)}mo`;
};

export default function TaskList() {
  const { data: tasks, isLoading, isError, refetch } = useTaskList();
  const [selectedRow, setSelectedRow] = useState<number | null>(null);
  const [formMode, setFormMode] = useState<'create' | 'edit' | null>(null);
  const [statusMsg, setStatusMsg] = useState<string | null>(null);

  const deleteTask = useDeleteTask();
  const completeTask = useCompleteTask();
  const uncompleteTask = useUncompleteTask();
  const startTask = useStartTask();
  const stopTask = useStopTask();

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement || e.target instanceof HTMLSelectElement) return;

      if (e.key === 'a') {
        e.preventDefault();
        setFormMode('create');
        setSelectedRow(null);
      }
      
      if (e.key === 'Escape') {
        setFormMode(null);
        setSelectedRow(null);
      }

      if (selectedRow !== null && tasks && tasks[selectedRow]) {
        if (e.key === 'Enter' || e.key === 'e') {
           e.preventDefault();
           setFormMode('edit');
        }
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [selectedRow, tasks, formMode]);

  useEffect(() => {
    if (statusMsg) {
      const timer = setTimeout(() => setStatusMsg(null), 2000);
      return () => clearTimeout(timer);
    }
  }, [statusMsg]);

  const handleAction = (action: () => void, msg: string) => {
    action();
    setStatusMsg(msg);
  };

  const handleDelete = (uuid: string) => {
    if (window.confirm("Delete task?")) {
      deleteTask.mutate(uuid, {
        onSuccess: () => setStatusMsg('Deleted')
      });
      setSelectedRow(null);
    }
  };

  if (isLoading) {
    return (
      <div className="flex justify-center items-center h-full text-text-primary">
        Loading...
      </div>
    );
  }

  if (isError) {
    return (
      <div className="flex flex-col justify-center items-center h-full gap-4 text-red-500">
        <p>Error loading tasks. Check server connection.</p>
        <button 
          onClick={() => refetch()}
          className="px-4 py-2 border border-red-500 hover:bg-red-500 hover:text-white transition-colors"
        >
          Retry
        </button>
      </div>
    );
  }

  if (!tasks || tasks.length === 0) {
    return (
      <div className="w-full h-full flex flex-col">
         {formMode === 'create' && (
          <TaskForm mode="create" onClose={() => setFormMode(null)} />
        )}
        <div className="flex-grow flex justify-center items-center text-text-primary opacity-60">
          No tasks. Press 'a' to add.
        </div>
      </div>
    );
  }

  return (
    <div className="w-full h-full overflow-auto flex flex-col">
      <div className="flex justify-between items-center px-2 py-1 border-b border-border bg-bg-primary sticky top-0 z-20">
        <div className="font-mono text-xs opacity-60">Tasks: {tasks.length}</div>
        <div className="flex gap-2">
            {statusMsg && <span className="text-green-500 font-mono text-xs">{statusMsg}</span>}
            <button 
              onClick={() => setFormMode('create')}
              className="px-2 py-0 border border-border hover:border-accent text-xs font-mono"
            >
              [+] Add
            </button>
        </div>
      </div>

      {formMode === 'create' && (
        <TaskForm mode="create" onClose={() => setFormMode(null)} />
      )}

      <table className="w-full text-left border-collapse font-mono text-sm" data-testid="task-list">
        <thead className="sticky top-8 bg-bg-primary z-10 border-b border-border">
          <tr>
            <th className="p-2 w-12 text-right">ID</th>
            <th className="p-2 w-16">Age</th>
            <th className="p-2">Description</th>
            <th className="p-2 w-32">Project</th>
            <th className="p-2 w-48">Tags</th>
            <th className="p-2 w-12 text-center">Pri</th>
            <th className="p-2 w-16">Due</th>
            <th className="p-2 w-16 text-right">Urg</th>
          </tr>
        </thead>
        <tbody>
          {tasks.map((task: TaskInfo, index: number) => {
            const isSelected = selectedRow === index;
            const isEditing = isSelected && formMode === 'edit';
            const rowId = index + 1;
            const urgency = task.urgency.toFixed(1);
            
            if (isEditing) {
              return (
                <TaskForm 
                  key={task.uuid} 
                  mode="edit" 
                  task={task} 
                  onClose={() => setFormMode(null)} 
                />
              );
            }

            return (
              <tr
                key={task.uuid}
                data-testid="task-row"
                data-selected={isSelected}
                onClick={() => {
                   if (selectedRow !== index) {
                      setSelectedRow(index);
                      setFormMode(null);
                   }
                }}
                className={`
                  cursor-pointer border-b border-border relative
                  ${isSelected 
                    ? 'bg-accent text-bg-primary' 
                    : 'hover:bg-border text-text-primary'
                  }
                `}
              >
                <td className="p-2 text-right">{rowId}</td>
                <td className="p-2 opacity-80">{formatDuration(task.entry)}</td>
                <td className="p-2">
                  {task.is_active && <span className="mr-2">▶</span>}
                  {task.description}
                  {isSelected && !formMode && (
                    <div className="absolute right-2 top-1/2 transform -translate-y-1/2 flex gap-1 bg-bg-primary p-1 rounded border border-border shadow-lg z-20" onClick={(e) => e.stopPropagation()}>
                       <button 
                         className="px-1 text-xs border border-border hover:bg-accent hover:text-white text-text-primary"
                         onClick={() => setFormMode('edit')}
                         title="Edit (e)"
                       >
                         Edit
                       </button>
                       <button 
                         className="px-1 text-xs border border-border hover:bg-accent hover:text-white text-text-primary"
                         onClick={() => task.status === 'completed' ? handleAction(() => uncompleteTask.mutate(task.uuid), 'Uncompleted') : handleAction(() => completeTask.mutate(task.uuid), 'Completed')}
                         title={task.status === 'completed' ? "Uncomplete" : "Complete"}
                       >
                         {task.status === 'completed' ? 'Uncomp' : 'Comp'}
                       </button>
                       <button 
                         className="px-1 text-xs border border-border hover:bg-accent hover:text-white text-text-primary"
                         onClick={() => task.is_active ? handleAction(() => stopTask.mutate(task.uuid), 'Stopped') : handleAction(() => startTask.mutate(task.uuid), 'Started')}
                         title={task.is_active ? "Stop" : "Start"}
                       >
                         {task.is_active ? 'Stop' : 'Start'}
                       </button>
                       <button 
                         className="px-1 text-xs border border-red-500 text-red-500 hover:bg-red-500 hover:text-white"
                         onClick={() => handleDelete(task.uuid)}
                         title="Delete"
                       >
                         Del
                       </button>
                    </div>
                  )}
                </td>
                <td className="p-2 truncate max-w-[8rem]" title={task.project}>
                  {task.project || ''}
                </td>
                <td className="p-2 truncate max-w-[12rem]" title={task.tags?.join(', ')}>
                  {task.tags?.map(t => `+${t}`).join(' ')}
                </td>
                <td className="p-2 text-center">{task.priority?.[0]?.toUpperCase() || ''}</td>
                <td className="p-2 opacity-80">{formatDue(task.due)}</td>
                <td className="p-2 text-right">{urgency}</td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
