import { useState } from "react";
import FundingTable from "./funding-table";
import FundingCreateForm from "./funding-create";
import FundingChartSummary from "./funding-chart-summary";
import FundingSummaryCards from "./funding-summary-cards";
import { DateRangeFilter } from "@/components/common/date-range-filter";
import { useFundingDateFilter } from "@/hooks/use-funding-date-filter";

import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Plus, History } from "lucide-react";
import { useIsMobile } from "@/hooks/use-mobile";
import { useNavigate } from "react-router-dom";

export default function Funding() {
  const isMobile = useIsMobile();
  const navigate = useNavigate();
  const [createDialogVisible, setCreateDialogVisible] = useState(false);
  const [refreshTrigger, setRefreshTrigger] = useState(0);
  const { dateRange, setDateRange } = useFundingDateFilter();

  const onCreated = () => {
    setRefreshTrigger((prev) => prev + 1);
    setCreateDialogVisible(false);
  };
  const onCancelCreate = () => {
    setCreateDialogVisible(false);
  };
  const onTableDataChange = () => {
    setRefreshTrigger((prev) => prev + 1);
  };

  return (
    <div>
      <div className="mb-4 flex flex-col gap-4 md:flex-row md:justify-between md:items-center">
        <DateRangeFilter date={dateRange} setDate={setDateRange} />
        <Button onClick={() => setCreateDialogVisible(true)} className="w-full md:w-auto">
          <Plus className="mr-2 h-4 w-4" />
          Deposit / Withdraw
        </Button>
      </div>
      <div className="mb-4">
        <FundingSummaryCards
          refreshTrigger={refreshTrigger}
          startDate={dateRange?.from}
          endDate={dateRange?.to}
        />
      </div>
      <div className="mb-4">
        <FundingChartSummary
          refreshTrigger={refreshTrigger}
          startDate={dateRange?.from}
          endDate={dateRange?.to}
        />
      </div>

      {isMobile ? (
        <Button 
          variant="outline" 
          className="w-full"
          onClick={() => navigate("/funding-history")}
        >
          <History className="mr-2 h-4 w-4" />
          View Full Funding History
        </Button>
      ) : (
        <FundingTable 
          refreshTrigger={refreshTrigger} 
          onDataChange={onTableDataChange}
          startDate={dateRange?.from}
          endDate={dateRange?.to}
        />
      )}

      <Dialog open={createDialogVisible} onOpenChange={setCreateDialogVisible}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Deposit or Withdraw Fund</DialogTitle>
          </DialogHeader>
          <FundingCreateForm onCancel={onCancelCreate} onCreate={onCreated} />
        </DialogContent>
      </Dialog>
    </div>
  );
}
