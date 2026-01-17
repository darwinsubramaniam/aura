import * as React from "react";
import { Upload, FileSpreadsheet, AlertCircle } from "lucide-react";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

interface FileUploaderProps {
  fileName: string | null;
  error: string | null;
  onFileUpload: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

export function FileUploader({
  fileName,
  error,
  onFileUpload,
}: FileUploaderProps) {
  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4 w-full max-w-xl">
          <div className="grid w-full gap-2">
            <Label htmlFor="file-upload" className="sr-only">
              Choose File
            </Label>
            <div className="relative">
              <Input
                id="file-upload"
                type="file"
                accept=".csv, .xlsx, .xls"
                onChange={onFileUpload}
                className="hidden" // Hide default input
              />
              <Label
                htmlFor="file-upload"
                className={`flex flex-col items-center justify-center w-full h-32 border-2 border-dashed rounded-lg cursor-pointer bg-muted/50 hover:bg-muted/80 transition-colors ${fileName ? "border-primary/50 bg-primary/5" : "border-muted-foreground/25"}`}
              >
                <div className="flex flex-col items-center justify-center pt-5 pb-6 text-center px-4">
                  {fileName ? (
                    <>
                      <FileSpreadsheet className="w-8 h-8 mb-2 text-primary" />
                      <p className="mb-1 text-sm font-semibold text-foreground">
                        {fileName}
                      </p>
                      <p className="text-xs text-muted-foreground">
                        Click to replace file
                      </p>
                    </>
                  ) : (
                    <>
                      <Upload className="w-8 h-8 mb-2 text-muted-foreground" />
                      <p className="mb-1 text-sm font-semibold text-foreground">
                        Click to upload file
                      </p>
                      <p className="text-xs text-muted-foreground">
                        CSV, Excel (.xlsx, .xls)
                      </p>
                    </>
                  )}
                </div>
              </Label>
            </div>
          </div>
        </div>
      </div>

      {error && (
        <div className="rounded-md bg-destructive/15 p-3 text-sm text-destructive flex items-center gap-2">
          <AlertCircle className="h-4 w-4" />
          {error}
        </div>
      )}
    </div>
  );
}
