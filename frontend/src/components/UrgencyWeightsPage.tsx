import { useState } from 'react';
import type { UrgencyWeights } from '../hooks/usePreferences';

interface UrgencyWeightsPageProps {
  weights: UrgencyWeights;
  onSave: (weights: UrgencyWeights) => void;
  onReset: () => void;
  onBack: () => void;
}

interface WeightConfig {
  key: keyof UrgencyWeights;
  label: string;
  description: string;
}

const weightConfigs: WeightConfig[] = [
  { key: 'next', label: 'Next Tag', description: 'Task has "next" tag' },
  { key: 'due', label: 'Due Date', description: 'Overdue or due soon' },
  { key: 'priorityHigh', label: 'Priority H', description: 'High priority' },
  { key: 'active', label: 'Active', description: 'Task is started' },
  { key: 'priorityMedium', label: 'Priority M', description: 'Medium priority' },
  { key: 'age', label: 'Age', description: 'Older tasks score higher' },
  { key: 'priorityLow', label: 'Priority L', description: 'Low priority' },
  { key: 'waiting', label: 'Waiting', description: 'Waiting date set (penalty)' },
  { key: 'tags', label: 'Tags', description: 'Any tags present' },
  { key: 'project', label: 'Project', description: 'Task has a project' },
];

export default function UrgencyWeightsPage({
  weights,
  onSave,
  onReset,
  onBack,
}: UrgencyWeightsPageProps) {
  const [localWeights, setLocalWeights] = useState<UrgencyWeights>({ ...weights });
  const [hasChanges, setHasChanges] = useState(false);

  const handleChange = (key: keyof UrgencyWeights, value: string) => {
    const num = parseFloat(value);
    if (!isNaN(num)) {
      setLocalWeights(prev => ({ ...prev, [key]: num }));
      setHasChanges(true);
    }
  };

  const handleSave = () => {
    onSave(localWeights);
    setHasChanges(false);
  };

  return (
    <div className="flex-1 min-h-0 overflow-auto font-mono text-text-primary">
      <div className="max-w-xl mx-auto p-6 space-y-6">
        <section>
          <h2 className="text-accent font-bold text-xs uppercase tracking-widest mb-2">
            [ Urgency Weights ]
          </h2>
          <p className="text-xs opacity-60 mb-4">
            Customize how urgency is calculated. These weights affect how tasks are sorted when you select "urgency" sort.
          </p>
        </section>

        <div className="space-y-3">
          {weightConfigs.map(config => (
            <div key={config.key} className="flex items-center justify-between gap-4">
              <div className="flex-1">
                <div className="text-sm">{config.label}</div>
                <div className="text-xs opacity-50">{config.description}</div>
              </div>
              <input
                type="number"
                step="0.1"
                value={localWeights[config.key]}
                onChange={e => handleChange(config.key, e.target.value)}
                className="w-20 bg-bg-primary border border-border px-2 py-1 text-sm text-right focus:border-accent focus:outline-none"
              />
            </div>
          ))}
        </div>

        <div className="flex gap-2 pt-2">
          <button
            onClick={handleSave}
            disabled={!hasChanges}
            className="border border-accent text-accent hover:bg-accent hover:text-bg-primary px-4 py-1.5 text-sm transition-all disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Save Changes
          </button>
          <button
            onClick={onReset}
            className="border border-border hover:border-accent px-4 py-1.5 text-sm transition-colors"
          >
            Reset to Defaults
          </button>
          <button
            onClick={onBack}
            className="text-sm opacity-60 hover:opacity-100 hover:text-accent transition-all ml-auto"
          >
            ← Back
          </button>
        </div>

        <section className="border-t border-border pt-4 mt-4">
          <h3 className="text-xs font-bold uppercase tracking-widest mb-2 opacity-60">
            [ Taskwarrior Defaults ]
          </h3>
          <div className="text-xs opacity-50 space-y-1">
            <div>next: 15 | due: 12 | H: 6 | active: 4</div>
            <div>M: 3.9 | age: 2.0 | L: 1.8 | waiting: -3</div>
          </div>
        </section>
      </div>
    </div>
  );
}
