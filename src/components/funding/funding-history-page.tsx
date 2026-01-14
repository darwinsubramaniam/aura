import { useState, useEffect } from "react";
import FundingTable from "./funding-table";
import { DateRangeFilter } from "@/components/common/date-range-filter";
import { useFundingDateFilter } from "@/hooks/use-funding-date-filter";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@/components/ui/dialog";
import { ArrowLeft, Smartphone } from "lucide-react";
import { useNavigate } from "react-router-dom";

export default function FundingHistoryPage() {
  const navigate = useNavigate();
  const { dateRange, setDateRange } = useFundingDateFilter();
  const [showOrientationWarning, setShowOrientationWarning] = useState(false);

  // Check orientation on mount and resize
  useEffect(() => {
    const checkOrientation = () => {
      // Check if device is mobile (rough check) and in portrait mode
      const isPortrait = window.innerHeight > window.innerWidth;
      const isMobile = window.innerWidth < 768; // Standard mobile breakpoint
      
      if (isMobile && isPortrait) {
        setShowOrientationWarning(true);
      } else {
        setShowOrientationWarning(false);
      }
    };

    checkOrientation();
    window.addEventListener("resize", checkOrientation);
    return () => window.removeEventListener("resize", checkOrientation);
  }, []);

  return (
    <div className="flex flex-col gap-4">
      {/* Navigation Header */}
      <div className="flex items-center gap-2">
        <Button variant="ghost" size="icon" onClick={() => navigate("/funding")}>
          <ArrowLeft className="h-5 w-5" />
        </Button>
        <h1 className="text-xl font-bold">Funding History</h1>
      </div>

      <div className="mb-4">
        <DateRangeFilter date={dateRange} setDate={setDateRange} />
      </div>

      <FundingTable
        startDate={dateRange?.from}
        endDate={dateRange?.to}
        // We can pass a refresh trigger if needed, but for simple viewing it's okay
      />

      {/* Orientation Warning Dialog */}
      <Dialog open={showOrientationWarning} onOpenChange={setShowOrientationWarning}>
        <DialogContent className="sm:max-w-md">
          <DialogHeader>
            <DialogTitle className="flex items-center gap-2">
              <Smartphone className="h-5 w-5 rotate-90" />
              Better Viewing Experience
            </DialogTitle>
            <DialogDescription>
              For the best experience viewing the data table, please rotate your device to landscape mode.
            </DialogDescription>
          </DialogHeader>
          <div className="flex justify-end">
            <Button variant="outline" onClick={() => setShowOrientationWarning(false)}>
              Dismiss
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
