import * as React from "react";
import {
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import { Pencil, Check, Trash2 } from "lucide-react";
import { cn } from "@/lib/utils";
import { Fiat } from "@/lib/models/fiat";
import { ProcessedRow } from "./types";

interface ProcessedDataTableProps {
  data: ProcessedRow[];
  fiats: Fiat[];
  onRemoveRow: (index: number) => void;
  onUpdateRowCurrency: (index: number, fiatId: string) => void;
  page: number;
  pageSize: number;
}

export function ProcessedDataTable({
  data,
  fiats,
  onRemoveRow,
  onUpdateRowCurrency,
  page,
  pageSize,
}: ProcessedDataTableProps) {
  const [openRowIndex, setOpenRowIndex] = React.useState<number | null>(null);

  return (
    <>
      <TableHeader className="bg-muted/50">
        <TableRow>
          <TableHead>Date</TableHead>
          <TableHead>Amount</TableHead>
          <TableHead>Currency</TableHead>
          <TableHead>Kind</TableHead>
          <TableHead>Exchange</TableHead>
          <TableHead className="w-[50px]"></TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {data.map((row, rowIndex) => {
          // Calculate the real index in the full dataset if needed, 
          // but here we are receiving the paginated chunk? 
          // Wait, the parent was slicing the data. 
          // If the parent slices the data, then 'rowIndex' is 0..pageSize.
          // But we need the 'realIndex' to update/remove correctly in the parent state.
          // So we need to reconstruct the real index or pass the sliced data with original indices.
          // The ProcessedRow has `_originalIndex` but that tracks index in the raw file.
          // For removing from 'previewData' array, we need the index in 'previewData'.
          
          const realIndex = (page - 1) * pageSize + rowIndex;

          return (
            <TableRow key={rowIndex}>
              <TableCell>
                <div className="flex flex-col">
                  <span
                    className={
                      !row.isDateValid ? "text-destructive font-medium" : ""
                    }
                  >
                    {row.date}
                  </span>
                  {!row.isDateValid && (
                    <span className="text-[10px] text-muted-foreground">
                      Original: {row.originalDate}
                    </span>
                  )}
                </div>
              </TableCell>
              <TableCell>{row.amount}</TableCell>
              <TableCell>
                {row.isCurrencyValid ? (
                  row.currency
                ) : (
                  <div className="flex items-center gap-2">
                    <div className="flex flex-col">
                      <span className="text-destructive font-medium">
                        {row.currency !== "-" ? row.currency : "Missing"}
                      </span>
                      <span className="text-[10px] text-destructive/80">
                        Unknown/Invalid
                      </span>
                    </div>

                    <Popover
                      open={openRowIndex === realIndex}
                      onOpenChange={(open) =>
                        setOpenRowIndex(open ? realIndex : null)
                      }
                      modal={true}
                    >
                      <PopoverTrigger asChild>
                        <Button
                          variant="ghost"
                          size="icon"
                          className="h-6 w-6"
                        >
                          <Pencil className="h-3 w-3" />
                        </Button>
                      </PopoverTrigger>
                      <PopoverContent className="p-0 w-[200px]" align="start">
                        <Command
                          filter={(value, search) => {
                            if (
                              value.toLowerCase().includes(search.toLowerCase())
                            )
                              return 1;
                            return 0;
                          }}
                        >
                          <CommandInput
                            placeholder="Search currency..."
                            className="h-9"
                          />
                          <CommandList>
                            <CommandEmpty>No currency found.</CommandEmpty>
                            <CommandGroup className="max-h-[200px] overflow-y-auto">
                              {fiats.map((fiat) => (
                                <CommandItem
                                  key={fiat.id}
                                  value={`${fiat.symbol} ${fiat.name}`}
                                  onSelect={() => {
                                    onUpdateRowCurrency(
                                      realIndex,
                                      String(fiat.id)
                                    );
                                    setOpenRowIndex(null);
                                  }}
                                >
                                  {fiat.symbol} - {fiat.name}
                                  <Check
                                    className={cn(
                                      "ml-auto h-4 w-4",
                                      String(row.fiatId) === String(fiat.id)
                                        ? "opacity-100"
                                        : "opacity-0"
                                    )}
                                  />
                                </CommandItem>
                              ))}
                            </CommandGroup>
                          </CommandList>
                        </Command>
                      </PopoverContent>
                    </Popover>
                  </div>
                )}
              </TableCell>
              <TableCell className="capitalize">{row.kind}</TableCell>
              <TableCell>{row.exchange || "-"}</TableCell>
              <TableCell>
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-8 w-8 text-muted-foreground hover:text-destructive"
                  onClick={() => onRemoveRow(realIndex)}
                >
                  <Trash2 className="h-4 w-4" />
                </Button>
              </TableCell>
            </TableRow>
          );
        })}
      </TableBody>
    </>
  );
}
