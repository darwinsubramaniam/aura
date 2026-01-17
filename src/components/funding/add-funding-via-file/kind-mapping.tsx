import { Plus, X } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { RampKind } from "./types";

interface KindMappingProps {
  distinctKindValues: string[];
  kindMapping: Record<string, RampKind>;
  updateKindMapping: (value: string, kind: RampKind) => void;
}

export function KindMapping({
  distinctKindValues,
  kindMapping,
  updateKindMapping,
}: KindMappingProps) {
  if (distinctKindValues.length === 0) return null;

  return (
    <div className="grid gap-6 md:grid-cols-2">
      {/* Deposit Mapping Group */}
      <MappingGroup
        title="Map to Deposit"
        titleColorClass="text-green-600"
        targetKind="deposit"
        opposingKind="withdraw"
        distinctValues={distinctKindValues}
        mapping={kindMapping}
        onUpdate={updateKindMapping}
        badgeColorClass="bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300 border-green-200 dark:border-green-800"
        hoverColorClass="hover:bg-green-200 dark:hover:bg-green-800"
      />

      {/* Withdraw Mapping Group */}
      <MappingGroup
        title="Map to Withdraw"
        titleColorClass="text-red-600"
        targetKind="withdraw"
        opposingKind="deposit"
        distinctValues={distinctKindValues}
        mapping={kindMapping}
        onUpdate={updateKindMapping}
        badgeColorClass="bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-300 border-red-200 dark:border-red-800"
        hoverColorClass="hover:bg-red-200 dark:hover:bg-red-800"
      />
    </div>
  );
}

interface MappingGroupProps {
  title: string;
  titleColorClass: string;
  targetKind: RampKind;
  opposingKind: RampKind;
  distinctValues: string[];
  mapping: Record<string, RampKind>;
  onUpdate: (value: string, kind: RampKind) => void;
  badgeColorClass: string;
  hoverColorClass: string;
}

function MappingGroup({
  title,
  titleColorClass,
  targetKind,
  opposingKind,
  distinctValues,
  mapping,
  onUpdate,
  badgeColorClass,
  hoverColorClass,
}: MappingGroupProps) {
  const availableValues = distinctValues.filter(
    (v) => mapping[v] !== targetKind
  );

  const mappedValues = distinctValues.filter((v) => mapping[v] === targetKind);

  return (
    <div className="space-y-4 border p-4 rounded-lg bg-background">
      <div className="flex flex-col gap-4">
        <div className="flex items-center justify-between">
          <h3 className={`font-medium text-sm ${titleColorClass}`}>{title}</h3>
        </div>

        <div className="flex gap-2">
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                variant="outline"
                size="sm"
                className="h-8 text-xs px-2 shrink-0"
              >
                <Plus className="h-3 w-3 mr-1" />
                Select Value
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent className="w-[200px] max-h-[300px]">
              {availableValues.length === 0 ? (
                <div className="p-2 text-xs text-muted-foreground text-center">
                  No values available
                </div>
              ) : (
                availableValues
                  .sort((a, b) => {
                    const aDisabled = mapping[a] === opposingKind;
                    const bDisabled = mapping[b] === opposingKind;
                    if (aDisabled && !bDisabled) return 1;
                    if (!aDisabled && bDisabled) return -1;
                    return a.localeCompare(b);
                  })
                  .map((val) => {
                    const isOpposing = mapping[val] === opposingKind;
                    return (
                      <DropdownMenuItem
                        key={val}
                        disabled={isOpposing}
                        onClick={() =>
                          !isOpposing && onUpdate(val, targetKind)
                        }
                        className={
                          isOpposing ? "text-muted-foreground opacity-70" : ""
                        }
                      >
                        <div className="flex flex-col">
                          <span>{val}</span>
                          {isOpposing && (
                            <span className="text-[10px] italic">
                              Mapped to {opposingKind}
                            </span>
                          )}
                        </div>
                      </DropdownMenuItem>
                    );
                  })
              )}
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>

      <div className="flex flex-wrap gap-2 min-h-[40px] p-2 bg-muted/20 rounded-md">
        {mappedValues.length === 0 && (
          <span className="text-xs text-muted-foreground italic p-1">
            No values mapped to {targetKind}
          </span>
        )}
        {mappedValues.map((val) => (
          <div
            key={val}
            className={`flex items-center gap-1 px-2 py-1 rounded-full text-xs border ${badgeColorClass}`}
          >
            <span className="truncate max-w-[150px]" title={val}>
              {val}
            </span>
            <button
              onClick={() => onUpdate(val, "ignore")}
              className={`${hoverColorClass} rounded-full p-0.5 cursor-pointer`}
            >
              <X className="h-3 w-3" />
            </button>
          </div>
        ))}
      </div>
    </div>
  );
}
