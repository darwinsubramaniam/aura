import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';

export function convertToCSV<T extends Record<string, any>>(
  data: T[],
  columns: { key: keyof T; header: string }[]
): string {
  if (!data || data.length === 0) {
    return "";
  }

  const headerRow = columns.map((col) => `"${col.header}"`).join(",");
  
  const bodyRows = data.map((row) => {
    return columns
      .map((col) => {
        const value = row[col.key];
        const stringValue = value === null || value === undefined ? "" : String(value);
        // Escape double quotes by doubling them
        return `"${stringValue.replace(/"/g, '""')}"`;
      })
      .join(",");
  });

  return [headerRow, ...bodyRows].join("\n");
}

export async function saveCSV(csvContent: string, defaultFilename: string): Promise<boolean> {
  // Try Tauri Native Save Dialog first
  try {
    // Check if running in Tauri environment (rudimentary check or try/catch)
    // The import from @tauri-apps/plugin-dialog might throw if not in Tauri or just fail at runtime
    const filePath = await save({
      defaultPath: defaultFilename,
      filters: [{
        name: 'CSV File',
        extensions: ['csv']
      }]
    });

    if (filePath) {
      await writeTextFile(filePath, csvContent);
      return true;
    }
    return false; // User cancelled
  } catch (e) {
    console.warn("Tauri save dialog failed or not available, falling back to browser download:", e);
    // Fallback to browser download if Tauri API fails (e.g. in web mode)
    downloadCSV(csvContent, defaultFilename);
    return true;
  }
}

function downloadCSV(csvContent: string, filename: string) {
  const blob = new Blob([csvContent], { type: "text/csv;charset=utf-8;" });
  const link = document.createElement("a");
  if (link.download !== undefined) {
    const url = URL.createObjectURL(blob);
    link.setAttribute("href", url);
    link.setAttribute("download", filename);
    link.style.visibility = "hidden";
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  }
}
