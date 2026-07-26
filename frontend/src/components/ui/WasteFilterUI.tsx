import React, { useState } from 'react';
import { Filter, X } from 'lucide-react';
import { cn } from '@/lib/cn';
import { WasteFilter, FilterPreset } from '@/lib/wasteFilterManager';

interface WasteFilterUIProps {
  filters: WasteFilter;
  onFilterChange: (filters: Partial<WasteFilter>) => void;
  onClear: () => void;
  presets: FilterPreset[];
  onApplyPreset: (presetId: string) => void;
  onSavePreset: (name: string) => void;
  isActive: boolean;
}

export const WasteFilterUI: React.FC<WasteFilterUIProps> = ({
  filters,
  onFilterChange,
  onClear,
  presets,
  onApplyPreset,
  onSavePreset,
  isActive,
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const [presetName, setPresetName] = useState('');

  const wasteTypes = ['Plastic', 'Metal', 'Paper', 'Glass', 'Organic'];
  const statuses = ['Pending', 'Verified', 'Transferred', 'Processed'];

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-2">
        <button
          onClick={() => setIsOpen(!isOpen)}
          aria-expanded={isOpen}
          aria-controls="waste-filter-panel"
          className={cn(
            'flex items-center gap-2 px-3 py-2 rounded-lg border transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
            isActive
              ? 'bg-blue-50 border-blue-300 text-blue-700'
              : 'bg-white border-gray-300 text-gray-700 hover:border-gray-400'
          )}
        >
          <Filter className="w-4 h-4" aria-hidden="true" />
          <span>Filters</span>
          {isActive && (
            <span
              className="ml-1 px-2 py-0.5 bg-blue-200 rounded-full text-xs font-medium"
              aria-label="filters active"
            >
              Active
            </span>
          )}
        </button>
        {isActive && (
          <button
            onClick={onClear}
            aria-label="Clear all filters"
            className="p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded-lg transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          >
            <X className="w-4 h-4" aria-hidden="true" />
          </button>
        )}
      </div>

      {isOpen && (
        <div id="waste-filter-panel" className="border border-gray-200 rounded-lg p-4 space-y-4 bg-gray-50">
          {/* Waste Type Filter */}
          <fieldset>
            <legend className="text-sm font-medium text-gray-700 mb-2">Waste Type</legend>
            <div className="flex flex-wrap gap-2">
              {wasteTypes.map((type) => {
                const selected = filters.wasteType?.includes(type) ?? false;
                return (
                  <button
                    key={type}
                    onClick={() => {
                      const current = filters.wasteType || [];
                      const updated = current.includes(type)
                        ? current.filter((t) => t !== type)
                        : [...current, type];
                      onFilterChange({ wasteType: updated });
                    }}
                    aria-pressed={selected}
                    className={cn(
                      'px-3 py-1 rounded-full text-sm transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
                      selected
                        ? 'bg-blue-500 text-white'
                        : 'bg-white border border-gray-300 text-gray-700 hover:border-gray-400'
                    )}
                  >
                    {type}
                  </button>
                );
              })}
            </div>
          </fieldset>

          {/* Status Filter */}
          <fieldset>
            <legend className="text-sm font-medium text-gray-700 mb-2">Status</legend>
            <div className="flex flex-wrap gap-2">
              {statuses.map((status) => {
                const selected = filters.status?.includes(status) ?? false;
                return (
                  <button
                    key={status}
                    onClick={() => {
                      const current = filters.status || [];
                      const updated = current.includes(status)
                        ? current.filter((s) => s !== status)
                        : [...current, status];
                      onFilterChange({ status: updated });
                    }}
                    aria-pressed={selected}
                    className={cn(
                      'px-3 py-1 rounded-full text-sm transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
                      selected
                        ? 'bg-green-500 text-white'
                        : 'bg-white border border-gray-300 text-gray-700 hover:border-gray-400'
                    )}
                  >
                    {status}
                  </button>
                );
              })}
            </div>
          </fieldset>

          {/* Weight Range Filter */}
          <div>
            <p id="weight-range-label" className="text-sm font-medium text-gray-700 mb-2">
              Weight Range (kg)
            </p>
            <div className="flex gap-2" role="group" aria-labelledby="weight-range-label">
              <div className="flex-1">
                <label htmlFor="weight-min" className="sr-only">Minimum weight in kg</label>
                <input
                  id="weight-min"
                  type="number"
                  placeholder="Min"
                  value={filters.weight?.min || ''}
                  onChange={(e) =>
                    onFilterChange({
                      weight: {
                        min: parseFloat(e.target.value) || 0,
                        max: filters.weight?.max || Infinity,
                      },
                    })
                  }
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                />
              </div>
              <div className="flex-1">
                <label htmlFor="weight-max" className="sr-only">Maximum weight in kg</label>
                <input
                  id="weight-max"
                  type="number"
                  placeholder="Max"
                  value={filters.weight?.max === Infinity ? '' : filters.weight?.max || ''}
                  onChange={(e) =>
                    onFilterChange({
                      weight: {
                        min: filters.weight?.min || 0,
                        max: parseFloat(e.target.value) || Infinity,
                      },
                    })
                  }
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                />
              </div>
            </div>
          </div>

          {/* Verification Status Filter */}
          <div>
            <label htmlFor="verification-status" className="block text-sm font-medium text-gray-700 mb-2">
              Verification Status
            </label>
            <select
              id="verification-status"
              value={filters.verificationStatus || 'all'}
              onChange={(e) =>
                onFilterChange({ verificationStatus: e.target.value as 'verified' | 'unverified' | 'all' })
              }
              className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            >
              <option value="all">All</option>
              <option value="verified">Verified</option>
              <option value="unverified">Unverified</option>
            </select>
          </div>

          {/* Save Preset */}
          <div className="flex gap-2">
            <label htmlFor="preset-name-input" className="sr-only">Preset name</label>
            <input
              id="preset-name-input"
              type="text"
              placeholder="Preset name"
              value={presetName}
              onChange={(e) => setPresetName(e.target.value)}
              className="flex-1 px-3 py-2 border border-gray-300 rounded-lg text-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            />
            <button
              onClick={() => {
                if (presetName.trim()) {
                  onSavePreset(presetName);
                  setPresetName('');
                }
              }}
              disabled={!presetName.trim()}
              aria-label="Save current filters as a preset"
              className="px-3 py-2 bg-blue-500 text-white rounded-lg text-sm hover:bg-blue-600 disabled:opacity-50 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            >
              Save
            </button>
          </div>
        </div>
      )}

      {/* Presets */}
      {presets.length > 0 && (
        <div className="space-y-2">
          <p id="presets-label" className="text-sm font-medium text-gray-700">Quick Presets</p>
          <div className="flex flex-wrap gap-2" role="group" aria-labelledby="presets-label">
            {presets.map((preset) => (
              <button
                key={preset.id}
                onClick={() => onApplyPreset(preset.id)}
                aria-label={`Apply preset: ${preset.name}`}
                className="px-3 py-1 bg-gray-200 text-gray-700 rounded-full text-sm hover:bg-gray-300 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
              >
                {preset.name}
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default WasteFilterUI;
