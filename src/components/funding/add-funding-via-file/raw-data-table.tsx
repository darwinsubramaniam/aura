import {
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { format, parse, isValid } from "date-fns";
import { ColumnMapping, DateSettings, COMMON_DATE_FORMATS } from "./types";

interface RawDataTableProps {
  headers: string[];
  data: any[];
  mapping: ColumnMapping;
  dateSettings: DateSettings;
}

export function RawDataTable({
  headers,
  data,
  mapping,
  dateSettings,
}: RawDataTableProps) {
  const parseRowDate = (raw: any): Date | null => {
    if (!raw) return null;

    try {
      if (dateSettings.type === "excel") {
        const serial = Number(raw);
        if (isNaN(serial)) return null;
        const excelBaseDate = new Date(Date.UTC(1899, 11, 30));
        return new Date(excelBaseDate.getTime() + serial * 86400000);
      }

      if (dateSettings.type === "timestamp_s") {
        const ts = Number(raw);
        if (isNaN(ts)) return null;
        return new Date(ts * 1000);
      }

      if (dateSettings.type === "timestamp_ms") {
        const ts = Number(raw);
        if (isNaN(ts)) return null;
        return new Date(ts);
      }

      const strVal = String(raw).trim();

      if (dateSettings.type === "string" && dateSettings.formatStr) {
        const parsed = parse(strVal, dateSettings.formatStr, new Date());
        if (isValid(parsed)) return parsed;
        return null;
      }

      let parsed = new Date(strVal);
      if (isValid(parsed)) return parsed;

      for (const fmt of COMMON_DATE_FORMATS) {
        parsed = parse(strVal, fmt, new Date());
        if (isValid(parsed)) return parsed;
      }

      return null;
    } catch (e) {
      return null;
    }
  };

  return (
    <>
      <TableHeader className="bg-muted/50">
        <TableRow>
          {headers.map((header, index) => (
            <TableHead key={index} className="font-bold whitespace-nowrap">
              <div className="flex flex-col gap-1">
                <span>{header}</span>
                {mapping.date === header && (
                  <span className="text-[10px] text-blue-500 font-normal">
                    Date
                  </span>
                )}
                {mapping.amount === header && (
                  <span className="text-[10px] text-green-500 font-normal">
                    Amount
                  </span>
                )}
                {mapping.currency === header && (
                  <span className="text-[10px] text-orange-500 font-normal">
                    Currency
                  </span>
                )}
                {mapping.kind === header && (
                  <span className="text-[10px] text-purple-500 font-normal">
                    Kind
                  </span>
                )}
              </div>
            </TableHead>
          ))}
        </TableRow>
      </TableHeader>
      <TableBody>
        {data.map((row, rowIndex) => (
          <TableRow key={rowIndex}>
            {headers.map((header, colIndex) => {
              const cellValue = row[header];
              let displayValue =
                cellValue !== undefined ? String(cellValue) : "";
              let isError = false;

              if (mapping.date === header) {
                const parsed = parseRowDate(cellValue);
                if (parsed) {
                  displayValue = format(parsed, "yyyy-MM-dd");
                } else if (cellValue && dateSettings.type !== "auto") {
                  isError = true;
                }
              }

              return (
                <TableCell key={colIndex}>
                  {isError ? (
                    <div className="flex flex-col">
                      <span className="text-destructive font-medium">
                        {String(cellValue)}
                      </span>
                      <span className="text-[10px] text-destructive/80">
                        Invalid {dateSettings.type}
                      </span>
                    </div>
                  ) : (
                    displayValue
                  )}
                </TableCell>
              );
            })}
          </TableRow>
        ))}
      </TableBody>
    </>
  );
}
