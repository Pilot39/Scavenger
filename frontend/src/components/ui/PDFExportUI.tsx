import React, { useState } from 'react';
import { Download, FileText } from 'lucide-react';
import { cn } from '@/lib/cn';
import PDFExporter, { WasteData, PDFExportOptions } from '@/lib/pdfExporter';

interface PDFExportUIProps {
  wastes: WasteData[];
  analytics?: {
    totalWaste: number;
    wasteByType: Record<string, number>;
    verificationRate: number;
    averageWeight: number;
  };
  className?: string;
}

export const PDFExportUI: React.FC<PDFExportUIProps> = ({
  wastes,
  analytics,
  className,
}) => {
  const [isExporting, setIsExporting] = useState(false);
  const [exportType, setExportType] = useState<'waste' | 'analytics'>('waste');
  const [options, setOptions] = useState<PDFExportOptions>({
    includeMetadata: true,
    orientation: 'portrait',
    format: 'a4',
  });

  const handleExport = async () => {
    setIsExporting(true);
    try {
      const exporter = new PDFExporter(options);
      const timestamp = new Date().toISOString().split('T')[0];

      if (exportType === 'waste') {
        exporter.exportWasteData(wastes, {
          title: 'Waste Report',
          ...options,
        });
        exporter.save(`waste-report-${timestamp}.pdf`);
      } else if (analytics) {
        exporter.exportAnalytics(analytics, {
          title: 'Analytics Report',
          ...options,
        });
        exporter.save(`analytics-report-${timestamp}.pdf`);
      }
    } catch (error) {
      console.error('PDF export failed:', error);
    } finally {
      setIsExporting(false);
    }
  };

  return (
    <div className={cn('space-y-4 p-4 border border-gray-200 rounded-lg', className)}>
      <div className="flex items-center gap-2">
        <FileText className="w-5 h-5 text-blue-600" />
        <h3 className="font-semibold text-gray-900">Export to PDF</h3>
      </div>

      {/* Export Type Selection */}
      <div className="space-y-2">
        <label className="block text-sm font-medium text-gray-700">
          Export Type
        </label>
        <div className="flex gap-2">
          <button
            onClick={() => setExportType('waste')}
            className={cn(
              'flex-1 px-3 py-2 rounded-lg text-sm font-medium transition-colors',
              exportType === 'waste'
                ? 'bg-blue-500 text-white'
                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
            )}
          >
            Waste Data
          </button>
          {analytics && (
            <button
              onClick={() => setExportType('analytics')}
              className={cn(
                'flex-1 px-3 py-2 rounded-lg text-sm font-medium transition-colors',
                exportType === 'analytics'
                  ? 'bg-blue-500 text-white'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              )}
            >
              Analytics
            </button>
          )}
        </div>
      </div>

      {/* Format Options */}
      <div className="grid grid-cols-2 gap-4">
        <div className="space-y-2">
          <label className="block text-sm font-medium text-gray-700">
            Orientation
          </label>
          <select
            value={options.orientation}
            onChange={(e) =>
              setOptions({
                ...options,
                orientation: e.target.value as 'portrait' | 'landscape',
              })
            }
            className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm"
          >
            <option value="portrait">Portrait</option>
            <option value="landscape">Landscape</option>
          </select>
        </div>

        <div className="space-y-2">
          <label className="block text-sm font-medium text-gray-700">
            Format
          </label>
          <select
            value={options.format}
            onChange={(e) =>
              setOptions({
                ...options,
                format: e.target.value as 'a4' | 'letter',
              })
            }
            className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm"
          >
            <option value="a4">A4</option>
            <option value="letter">Letter</option>
          </select>
        </div>
      </div>

      {/* Include Metadata Checkbox */}
      <label className="flex items-center gap-2 cursor-pointer">
        <input
          type="checkbox"
          checked={options.includeMetadata}
          onChange={(e) =>
            setOptions({
              ...options,
              includeMetadata: e.target.checked,
            })
          }
          className="w-4 h-4 rounded border-gray-300"
        />
        <span className="text-sm text-gray-700">Include Summary Metadata</span>
      </label>

      {/* Export Button */}
      <button
        onClick={handleExport}
        disabled={isExporting || (exportType === 'waste' && wastes.length === 0)}
        className={cn(
          'w-full flex items-center justify-center gap-2 px-4 py-2 rounded-lg font-medium transition-colors',
          isExporting || (exportType === 'waste' && wastes.length === 0)
            ? 'bg-gray-300 text-gray-500 cursor-not-allowed'
            : 'bg-blue-600 text-white hover:bg-blue-700'
        )}
      >
        <Download className="w-4 h-4" />
        {isExporting ? 'Exporting...' : 'Export PDF'}
      </button>

      {exportType === 'waste' && wastes.length === 0 && (
        <p className="text-sm text-gray-500 text-center">
          No waste data available to export
        </p>
      )}
    </div>
  );
};

export default PDFExportUI;
