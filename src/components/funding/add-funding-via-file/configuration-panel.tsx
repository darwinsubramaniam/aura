import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import { Fiat } from "@/lib/models/fiat";
import { ColumnMapping, DateSettings, COMMON_DATE_FORMATS, RampKind } from "./types";
import { KindMapping } from "./kind-mapping";

interface ConfigurationPanelProps {
  headers: string[];
  mapping: ColumnMapping;
  updateMapping: (field: keyof ColumnMapping, value: string) => void;
  dateSettings: DateSettings;
  setDateSettings: (settings: DateSettings) => void;
  fiats: Fiat[];
  defaultFiatId: string;
  setDefaultFiatId: (id: string) => void;
  exchangeName: string;
  setExchangeName: (name: string) => void;
  distinctKindValues: string[];
  kindMapping: Record<string, RampKind>;
  updateKindMapping: (value: string, kind: RampKind) => void;
  onPreview: () => void;
}

export function ConfigurationPanel({
  headers,
  mapping,
  updateMapping,
  dateSettings,
  setDateSettings,
  fiats,
  defaultFiatId,
  setDefaultFiatId,
  exchangeName,
  setExchangeName,
  distinctKindValues,
  kindMapping,
  updateKindMapping,
  onPreview,
}: ConfigurationPanelProps) {
  return (
    <div className="space-y-6">
      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3 bg-muted/30 p-4 rounded-lg border">
        <div className="space-y-2">
          <Label>Date Column</Label>
          <div className="flex flex-col gap-2">
            <Select
              onValueChange={(v) => updateMapping("date", v)}
              value={mapping.date}
            >
              <SelectTrigger>
                <SelectValue placeholder="Select column" />
              </SelectTrigger>
              <SelectContent>
                {headers.map((h) => (
                  <SelectItem key={h} value={h}>
                    {h}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>

            {mapping.date && (
              <div className="p-2 border rounded bg-background flex flex-col gap-2">
                <Label className="text-xs text-muted-foreground">
                  Date Type
                </Label>
                <Select
                  value={dateSettings.type}
                  onValueChange={(v: any) =>
                    setDateSettings({ ...dateSettings, type: v })
                  }
                >
                  <SelectTrigger className="h-8 text-xs">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="auto">Auto Detect / String</SelectItem>
                    <SelectItem value="timestamp_s">
                      Unix Timestamp (Seconds)
                    </SelectItem>
                    <SelectItem value="timestamp_ms">
                      Unix Timestamp (Milliseconds)
                    </SelectItem>
                    <SelectItem value="excel">Excel Serial Date</SelectItem>
                    <SelectItem value="string">
                      String (Custom Format)
                    </SelectItem>
                  </SelectContent>
                </Select>

                {dateSettings.type === "string" && (
                  <>
                    <Label className="text-xs text-muted-foreground">
                      Format
                    </Label>
                    <Select
                      value={dateSettings.formatStr}
                      onValueChange={(v) =>
                        setDateSettings({ ...dateSettings, formatStr: v })
                      }
                    >
                      <SelectTrigger className="h-8 text-xs">
                        <SelectValue placeholder="Select Format" />
                      </SelectTrigger>
                      <SelectContent>
                        {COMMON_DATE_FORMATS.map((fmt) => (
                          <SelectItem key={fmt} value={fmt}>
                            {fmt}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </>
                )}
              </div>
            )}
          </div>
        </div>

        <div className="space-y-2">
          <Label>Amount Column</Label>
          <Select
            onValueChange={(v) => updateMapping("amount", v)}
            value={mapping.amount}
          >
            <SelectTrigger>
              <SelectValue placeholder="Select column" />
            </SelectTrigger>
            <SelectContent>
              {headers.map((h) => (
                <SelectItem key={h} value={h}>
                  {h}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <div className="space-y-2">
          <Label>Currency Column</Label>
          <div className="flex flex-col gap-2">
            <Select
              onValueChange={(v) => updateMapping("currency", v)}
              value={mapping.currency}
            >
              <SelectTrigger>
                <SelectValue placeholder="Select column (Optional)" />
              </SelectTrigger>
              <SelectContent>
                {headers.map((h) => (
                  <SelectItem key={h} value={h}>
                    {h}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>

            {!mapping.currency && (
              <div className="p-2 border rounded bg-background flex flex-col gap-2">
                <Label className="text-xs text-muted-foreground">
                  Default Currency (Fallback)
                </Label>
                <Select value={defaultFiatId} onValueChange={setDefaultFiatId}>
                  <SelectTrigger className="h-8 text-xs">
                    <SelectValue placeholder="Select Fiat" />
                  </SelectTrigger>
                  <SelectContent>
                    {fiats.map((f) => (
                      <SelectItem key={f.id} value={String(f.id)}>
                        {f.symbol} - {f.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            )}
          </div>
        </div>

        <div className="space-y-2">
          <Label>Type/Kind Column</Label>
          <Select
            onValueChange={(v) => updateMapping("kind", v)}
            value={mapping.kind}
          >
            <SelectTrigger>
              <SelectValue placeholder="Select column" />
            </SelectTrigger>
            <SelectContent>
              {headers.map((h) => (
                <SelectItem key={h} value={h}>
                  {h}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <div className="space-y-2">
          <Label>Exchange Name</Label>
          <Input
            placeholder="e.g. Coinbase, Kraken"
            value={exchangeName}
            onChange={(e) => setExchangeName(e.target.value)}
          />
        </div>

        <div className="flex items-end justify-start">
          <Button onClick={onPreview} className="w-full">
            Preview
          </Button>
        </div>
      </div>

      {mapping.kind && distinctKindValues.length > 0 && (
        <KindMapping
          distinctKindValues={distinctKindValues}
          kindMapping={kindMapping}
          updateKindMapping={updateKindMapping}
        />
      )}
    </div>
  );
}
