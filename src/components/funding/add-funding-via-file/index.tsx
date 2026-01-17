"use client";

import * as React from "react";
import * as XLSX from "xlsx";
import { ChevronLeft, ChevronRight } from "lucide-react";
import * as ScrollAreaPrimitive from "@radix-ui/react-scroll-area";
import { ScrollBar } from "@/components/ui/scroll-area";
import { parse, isValid, format } from "date-fns";

import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { FiatCommand } from "@/lib/services/fiat/fiat.command";
import { Fiat } from "@/lib/models/fiat";
import { FiatRampCommand } from "@/lib/services/funding/fiatRamp.command";
import { useNotification } from "@/components/common/NotificationProvider";
import { Progress } from "@/components/ui/progress";
import { listen } from "@tauri-apps/api/event";
import { useNavigate } from "react-router-dom";

import { FileUploader } from "./file-uploader";
import { ConfigurationPanel } from "./configuration-panel";
import { RawDataTable } from "./raw-data-table";
import { ProcessedDataTable } from "./processed-data-table";
import {
  ColumnMapping,
  DateSettings,
  RampKind,
  ProcessedRow,
  COMMON_DATE_FORMATS,
} from "./types";

interface FundingViaFileProps {
  className?: string;
}

export function AddFundingViaFile({ className }: FundingViaFileProps) {
  const navigate = useNavigate();
  const { showSuccess, showError } = useNotification();
  const [data, setData] = React.useState<any[]>([]);
  const [headers, setHeaders] = React.useState<string[]>([]);
  const [fileName, setFileName] = React.useState<string | null>(null);
  const [error, setError] = React.useState<string | null>(null);

  // Mapping State
  const [fiats, setFiats] = React.useState<Fiat[]>([]);
  const [mapping, setMapping] = React.useState<ColumnMapping>({
    date: "",
    amount: "",
    currency: "",
    kind: "",
  });
  const [dateSettings, setDateSettings] = React.useState<DateSettings>({
    type: "auto",
    formatStr: "",
  });
  const [kindMapping, setKindMapping] = React.useState<
    Record<string, RampKind>
  >({});
  const [distinctKindValues, setDistinctKindValues] = React.useState<string[]>(
    [],
  );
  const [exchangeName, setExchangeName] = React.useState("");
  const [defaultFiatId, setDefaultFiatId] = React.useState<string>("");

  const [activeTab, setActiveTab] = React.useState("configure");
  const [previewData, setPreviewData] = React.useState<ProcessedRow[]>([]);

  // Pagination State
  const [page, setPage] = React.useState(1);
  const pageSize = 50;

  const [importProgress, setImportProgress] = React.useState({
    processed: 0,
    total: 0,
  });
  const [isImporting, setIsImporting] = React.useState(false);

  React.useEffect(() => {
    FiatCommand.getAllCurrencies().then(setFiats).catch(console.error);

    const unlisten = listen<{ processed: number; total: number }>(
      "fiat-ramp-bulk-progress",
      (event) => {
        setImportProgress(event.payload);
      },
    );

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  React.useEffect(() => {
    setPage(1);
  }, [data, activeTab]);

  const currentData = activeTab === "configure" ? data : previewData;
  const totalPages = Math.ceil(currentData.length / pageSize);
  const paginatedData = currentData.slice(
    (page - 1) * pageSize,
    page * pageSize,
  );

  React.useEffect(() => {
    if (mapping.kind && data.length > 0) {
      const values = new Set<string>();
      data.forEach((row) => {
        const val = row[mapping.kind];
        if (val !== undefined && val !== null) {
          values.add(String(val));
        }
      });
      const distinct = Array.from(values);
      setDistinctKindValues(distinct);

      const newKindMapping: Record<string, RampKind> = {};
      distinct.forEach((v) => (newKindMapping[v] = "ignore"));
      setKindMapping(newKindMapping);
    } else {
      setDistinctKindValues([]);
      setKindMapping({});
    }
  }, [mapping.kind, data]);

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    setFileName(file.name);
    setError(null);

    setMapping({ date: "", amount: "", currency: "", kind: "" });
    setDateSettings({ type: "auto", formatStr: "" });
    setExchangeName("");
    setDefaultFiatId("");
    setKindMapping({});
    setActiveTab("configure");
    setPreviewData([]);

    const reader = new FileReader();
    reader.onload = (evt) => {
      try {
        const bstr = evt.target?.result;
        const wb = XLSX.read(bstr, { type: "array" });
        const wsname = wb.SheetNames[0];
        const ws = wb.Sheets[wsname];
        const data = XLSX.utils.sheet_to_json(ws, { header: 1 }) as any[][];

        if (data.length === 0) {
          setError("The file appears to be empty.");
          return;
        }

        const cols = data[0] as string[];
        const rows = data.slice(1).map((row) => {
          const rowData: Record<string, any> = {};
          cols.forEach((col, index) => {
            rowData[col] = row[index];
          });
          return rowData;
        });

        setHeaders(cols);
        setData(rows);
      } catch (err) {
        console.error(err);
        setError(
          "Failed to parse the file. Please ensure it is a valid CSV or Excel file.",
        );
      }
    };
    reader.onerror = () => {
      setError("Failed to read the file.");
    };
    reader.readAsArrayBuffer(file);
  };

  const updateMapping = (field: keyof ColumnMapping, value: string) => {
    setMapping((prev) => ({ ...prev, [field]: value }));
  };

  const updateKindMapping = (value: string, kind: RampKind) => {
    setKindMapping((prev) => ({ ...prev, [value]: kind }));
  };

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

  const handlePreview = () => {
    if (!mapping.date || !mapping.amount || !mapping.kind) {
      setError("Please map at least Date, Amount and Kind columns to preview.");
      return;
    }
    setError(null);

    const processed = data
      .map((row, index) => {
        const dateVal = row[mapping.date];
        
        // Ensure amount is absolute value
        const rawAmount = row[mapping.amount];
        let amountVal = rawAmount;
        if (rawAmount !== undefined && rawAmount !== null && rawAmount !== "") {
             const parsed = Number(rawAmount);
             if (!isNaN(parsed)) {
                 amountVal = Math.abs(parsed);
             }
        }

        const currencyVal = mapping.currency
          ? row[mapping.currency]
          : undefined;
        const kindVal = row[mapping.kind];

        let mappedKind: RampKind | undefined = undefined;
        if (kindVal !== undefined && kindVal !== null) {
          mappedKind = kindMapping[String(kindVal)];
        }

        if (mappedKind === "ignore" || !mappedKind) return null;

        const parsedDate = parseRowDate(dateVal);

        let resolvedCurrencySymbol = "";
        let fiatId = 0;

        if (currencyVal) {
          const match = fiats.find(
            (f) => f.symbol === currencyVal || f.name === currencyVal,
          );
          if (match) {
            fiatId = match.id;
            resolvedCurrencySymbol = match.symbol;
          } else {
            resolvedCurrencySymbol = currencyVal;
          }
        }

        if (fiatId === 0 && defaultFiatId && !currencyVal) {
          const match = fiats.find((f) => String(f.id) === defaultFiatId);
          if (match) {
            fiatId = match.id;
            resolvedCurrencySymbol = match.symbol;
          }
        }

        const isCurrencyValid = fiatId !== 0;

        // Cast kind to Exclude<RampKind, "ignore"> as we filtered above
        const validKind = mappedKind as Exclude<RampKind, "ignore">;

        return {
          _originalIndex: index,
          date: parsedDate ? format(parsedDate, "yyyy-MM-dd") : "Invalid Date",
          originalDate: dateVal,
          amount: amountVal,
          currency: resolvedCurrencySymbol || "-",
          kind: validKind,
          exchange: exchangeName,
          isDateValid: !!parsedDate,
          isCurrencyValid: isCurrencyValid,
          fiatId: fiatId,
        };
      })
      .filter(Boolean) as ProcessedRow[];

    if (processed.length === 0) {
      setError("No records found after filtering. Check your kind mappings.");
      return;
    }

    setPreviewData(processed);
    setActiveTab("preview");
  };

  const handleRemoveRow = (index: number) => {
    setPreviewData((prev) => prev.filter((_, i) => i !== index));
  };

  const handleUpdateRowCurrency = (index: number, fiatId: string) => {
    const fiat = fiats.find((f) => String(f.id) === fiatId);
    if (!fiat) return;

    setPreviewData((prev) =>
      prev.map((row, i) => {
        if (i !== index) return row;
        return {
          ...row,
          fiatId: fiat.id,
          currency: fiat.symbol,
          isCurrencyValid: true,
        };
      }),
    );
  };

  const handleImport = async () => {
    setIsImporting(true);
    setImportProgress({ processed: 0, total: previewData.length });

    try {
      const payload = [];
      for (const row of previewData) {
        if (!row.isDateValid) {
          throw new Error(`Row ${row._originalIndex + 1} has an invalid date.`);
        }
        if (!row.isCurrencyValid || !row.fiatId) {
          throw new Error(
            `Row ${row._originalIndex + 1} has an invalid or missing currency.`,
          );
        }

        payload.push({
          fiat_id: row.fiatId,
          fiat_amount: Number(row.amount),
          ramp_date: new Date(row.date),
          via_exchange: row.exchange || "Imported",
          kind: row.kind,
        });
      }

      await FiatRampCommand.createBulk(payload);
      showSuccess(`Successfully imported ${payload.length} records`);
      setTimeout(() => {
        navigate("/funding");
      }, 1000);
    } catch (e) {
      showError(`Import failed: ${e}`);
      setIsImporting(false);
    }
  };

  return (
    <div className={`space-y-6 ${className}`}>
      <FileUploader
        fileName={fileName}
        error={error}
        onFileUpload={handleFileUpload}
      />

      {headers.length > 0 && (
        <Tabs value={activeTab} onValueChange={setActiveTab} className="w-full">
          <div className="flex items-center justify-between mb-4">
            <TabsList>
              <TabsTrigger value="configure">Configuration</TabsTrigger>
              <TabsTrigger value="preview" disabled={previewData.length === 0}>
                Preview Data
              </TabsTrigger>
            </TabsList>
          </div>

          <TabsContent value="configure">
            <ConfigurationPanel
              headers={headers}
              mapping={mapping}
              updateMapping={updateMapping}
              dateSettings={dateSettings}
              setDateSettings={setDateSettings}
              fiats={fiats}
              defaultFiatId={defaultFiatId}
              setDefaultFiatId={setDefaultFiatId}
              exchangeName={exchangeName}
              setExchangeName={setExchangeName}
              distinctKindValues={distinctKindValues}
              kindMapping={kindMapping}
              updateKindMapping={updateKindMapping}
              onPreview={handlePreview}
            />
          </TabsContent>

          <TabsContent value="preview">
            {isImporting && (
              <div className="space-y-2 py-4 px-1">
                <div className="flex justify-between text-xs text-muted-foreground">
                  <span>Importing...</span>
                  <span>
                    {Math.round(
                      (importProgress.processed / importProgress.total) * 100,
                    )}
                    % ({importProgress.processed}/{importProgress.total})
                  </span>
                </div>
                <Progress
                  value={
                    (importProgress.processed / importProgress.total) * 100
                  }
                  className="h-2"
                />
              </div>
            )}

            <div className="flex items-center justify-end gap-2 mb-4">
              <div className="text-sm text-muted-foreground mr-auto">
                Ready to import {previewData.length} records.
              </div>
              <Button
                onClick={handleImport}
                disabled={
                  isImporting ||
                  previewData.length === 0 ||
                  previewData.some(
                    (row) => !row.isDateValid || !row.isCurrencyValid,
                  )
                }
              >
                {isImporting ? "Importing..." : "Confirm & Import"}
              </Button>
            </div>
          </TabsContent>
        </Tabs>
      )}

      {headers.length > 0 && (
        <div className="rounded-md border">
          <ScrollAreaPrimitive.Root className="relative overflow-hidden">
            <ScrollAreaPrimitive.Viewport className="h-full w-full rounded-[inherit]">
              <div className="pt-4 min-w-full">
                <table className="w-full caption-bottom text-sm">
                  {activeTab === "configure" ? (
                    <RawDataTable
                      headers={headers}
                      data={paginatedData}
                      mapping={mapping}
                      dateSettings={dateSettings}
                    />
                  ) : (
                    <ProcessedDataTable
                      data={paginatedData as ProcessedRow[]}
                      fiats={fiats}
                      onRemoveRow={handleRemoveRow}
                      onUpdateRowCurrency={handleUpdateRowCurrency}
                      page={page}
                      pageSize={pageSize}
                    />
                  )}
                </table>
              </div>
            </ScrollAreaPrimitive.Viewport>
            <ScrollBar
              orientation="horizontal"
              className="top-0 bottom-auto border-b border-t-0"
            />
          </ScrollAreaPrimitive.Root>

          {totalPages > 1 && (
            <div className="flex items-center justify-between px-4 py-4 border-t">
              <div className="text-xs text-muted-foreground">
                Showing {(page - 1) * pageSize + 1} to{" "}
                {Math.min(page * pageSize, currentData.length)} of{" "}
                {currentData.length} entries
              </div>
              <div className="flex items-center space-x-2">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setPage((p) => Math.max(1, p - 1))}
                  disabled={page === 1}
                  className="h-8 w-8 p-0"
                >
                  <ChevronLeft className="h-4 w-4" />
                </Button>
                <div className="text-xs font-medium">
                  Page {page} of {totalPages}
                </div>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setPage((p) => Math.min(totalPages, p + 1))}
                  disabled={page === totalPages}
                  className="h-8 w-8 p-0"
                >
                  <ChevronRight className="h-4 w-4" />
                </Button>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
