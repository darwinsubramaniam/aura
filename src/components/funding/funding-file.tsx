import { FundingViaFile } from "@/components/funding/funding-via-file";
import { Button } from "@/components/ui/button";
import { useNavigate } from "react-router-dom";
import { ArrowLeft } from "lucide-react";

export default function FundingFilePage() {
  const navigate = useNavigate();

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-4">
        <Button
          variant="ghost"
          size="icon"
          onClick={() => navigate("/funding")}
        >
          <ArrowLeft className="h-4 w-4" />
        </Button>
        <h1 className="text-2xl font-bold">Import Fiat Ramp File</h1>
      </div>

      <div className="p-4 border rounded-lg bg-card text-card-foreground shadow-sm">
        <FundingViaFile />
      </div>
    </div>
  );
}
