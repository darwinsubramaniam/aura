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

export function downloadCSV(csvContent: string, filename: string) {
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
