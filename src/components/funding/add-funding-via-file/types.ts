export interface ColumnMapping {
  date: string;
  amount: string;
  currency: string;
  kind: string;
}

export interface DateSettings {
  type: "auto" | "string" | "timestamp_s" | "timestamp_ms" | "excel";
  formatStr: string;
}

export type RampKind = "deposit" | "withdraw" | "ignore";

export const COMMON_DATE_FORMATS = [
  "yyyy-MM-dd",
  "dd/MM/yyyy",
  "MM/dd/yyyy",
  "dd-MM-yyyy",
  "yyyy/MM/dd",
  "dd MMM yyyy",
  "yyyy-MM-dd HH:mm:ss",
  "dd/MM/yyyy HH:mm:ss",
];

export interface ProcessedRow {
  _originalIndex: number;
  date: string;
  originalDate: any;
  amount: any;
  currency: string;
  kind: Exclude<RampKind, "ignore">; // It should only be deposit or withdraw in preview
  exchange: string;
  isDateValid: boolean;
  isCurrencyValid: boolean;
  fiatId: number;
}
