import { useState } from 'react';
import { useTaskList } from '../api/hooks';
import type { TaskInfo } from '../api/types';

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
      <div className="flex justify-center items-center h-full text-text-primary opacity-60">
        No tasks. Press 'a' to add.
      </div>
    );
  }

  return (
    <div className="w-full h-full overflow-auto">
      <table className="w-full text-left border-collapse font-mono text-sm" data-testid="task-list">
        <thead className="sticky top-0 bg-bg-primary z-10 border-b border-border">
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
            const rowId = index + 1;
            const urgency = task.urgency.toFixed(1);
            
            return (
              <tr
                key={task.uuid}
                data-testid="task-row"
                data-selected={isSelected}
                onClick={() => setSelectedRow(index)}
                className={`
                  cursor-pointer border-b border-border
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
