/**
 * useExportImport — issue #1072 (backward-compat re-export)
 *
 * The combined hook has been split into focused modules:
 *   - useExport  → frontend/src/hooks/useExport.ts
 *   - useImport  → frontend/src/hooks/useImport.ts
 *
 * This file re-exports both for any code that imports from the original
 * combined path, and is the only surviving artifact from the dead-code
 * cleanup.  New code should import directly from `useExport` or
 * `useImport`.
 *
 * Dead code removed in this cleanup:
 *   - XML format handler  (no call sites in the codebase)
 *   - XLS/XLSX format handler (no call sites in the codebase)
 *   - "legacy-csv" branch with a different column order (superseded)
 *   - Unreachable "dry-run preview" branch never wired to a UI component
 */
export { useExport } from './useExport';
export type { CsvRow, ExportOptions } from './useExport';

export { useImport } from './useImport';
export type { ImportState, CsvRecord } from './useImport';
