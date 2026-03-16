import type { TaskFilterParams } from '../api/types';

interface FilterBarProps {
  filter: TaskFilterParams;
  onFilterChange: (newFilter: TaskFilterParams) => void;
}

export default function FilterBar({ filter, onFilterChange }: FilterBarProps) {
  const handleChange = (key: keyof TaskFilterParams, value: string) => {
    const newFilter = { ...filter, [key]: value };
    if (!value) delete newFilter[key];
    onFilterChange(newFilter);
  };

  const hasFilters = Object.keys(filter).length > 0;

  return (
    <div className="shrink-0 w-full border-b border-border p-2 bg-bg-primary flex flex-wrap gap-4 items-center text-sm font-mono select-none">
      <div className="flex items-center gap-2">
        <span className="text-accent opacity-70">F:</span>
        
        <select
          value={filter.status || ''}
          onChange={(e) => handleChange('status', e.target.value)}
          className="bg-bg-primary border border-border text-text-primary px-1 py-0.5 focus:border-accent focus:outline-none"
          title="Filter by Status"
        >
          <option value="">[Status: All]</option>
          <option value="pending">Pending</option>
          <option value="completed">Completed</option>
          <option value="waiting">Waiting</option>
        </select>

        <div className="relative flex items-center">
          <span className="absolute left-2 opacity-50 text-xs">proj:</span>
          <input
            type="text"
            value={filter.project || ''}
            onChange={(e) => handleChange('project', e.target.value)}
            placeholder="..."
            data-filter="project"
            className="pl-10 pr-1 w-24 bg-bg-primary border border-border text-text-primary px-1 py-0.5 focus:border-accent focus:outline-none placeholder-opacity-20"
            title="Filter by Project"
          />
        </div>

        <div className="relative flex items-center">
          <span className="absolute left-2 opacity-50 text-xs">tag:</span>
          <input
            type="text"
            value={filter.tag || ''}
            onChange={(e) => handleChange('tag', e.target.value)}
            placeholder="..."
            className="pl-8 pr-1 w-24 bg-bg-primary border border-border text-text-primary px-1 py-0.5 focus:border-accent focus:outline-none placeholder-opacity-20"
            title="Filter by Tag"
          />
        </div>
      </div>

      <div className="flex items-center gap-2 ml-auto">
        <span className="text-accent opacity-70">S:</span>
        <select
          value={filter.sort_by || 'urgency'}
          onChange={(e) => handleChange('sort_by', e.target.value)}
          className="bg-bg-primary border border-border text-text-primary px-1 py-0.5 focus:border-accent focus:outline-none"
          title="Sort By"
        >
          <option value="urgency">[Sort: Urgency]</option>
          <option value="due">Due Date</option>
          <option value="priority">Priority</option>
          <option value="entry">Entry Date</option>
          <option value="modified">Modified</option>
          <option value="description">Description</option>
        </select>
      </div>

      {hasFilters && (
        <button
          onClick={() => onFilterChange({})}
          className="ml-2 text-xs text-red-500 hover:text-red-400 border border-red-500/30 px-1 hover:border-red-500 transition-colors"
          title="Clear all filters"
        >
          [x] Clear
        </button>
      )}
    </div>
  );
}
