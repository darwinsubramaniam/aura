import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { FiatRampCommand } from "@/lib/services/funding/fiatRamp.command";
import { FiatRampSummary } from "@/lib/models/fiatRamp";
import { useEffect, useState } from "react";
import { ArrowDownLeft, ArrowUpRight, Wallet } from "lucide-react";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";

interface FundingSummaryCardsProps {
  refreshTrigger?: number;
  startDate?: Date | null;
  endDate?: Date | null;
}

export default function FundingSummaryCards({
  refreshTrigger,
  startDate,
  endDate,
}: FundingSummaryCardsProps) {
  const [summary, setSummary] = useState<FiatRampSummary | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchSummary = async () => {
      try {
        setLoading(true);
        const data = await FiatRampCommand.getSummary(startDate || undefined, endDate || undefined);
        setSummary(data);
      } catch (error) {
        console.error("Failed to fetch funding summary:", error);
      } finally {
        setLoading(false);
      }
    };

    fetchSummary();
  }, [refreshTrigger, startDate, endDate]);

  const totalInvested =
    (summary?.total_deposit ?? 0) - (summary?.total_withdraw ?? 0);
  const currencySymbol = summary?.fiat_symbol ?? "$";

  return (
    <TooltipProvider>
      <div className="grid gap-4 md:grid-cols-3 mb-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              <Tooltip>
                <TooltipTrigger className="flex items-center gap-1 cursor-help">
                  Total Deposits
                </TooltipTrigger>
                <TooltipContent>
                  <p>Total funds deposited</p>
                </TooltipContent>
              </Tooltip>
            </CardTitle>
            <ArrowDownLeft className="h-4 w-4 text-green-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {loading
                ? "..."
                : `${currencySymbol} ${summary?.total_deposit?.toFixed(2) ?? "0.00"}`}
            </div>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              <Tooltip>
                <TooltipTrigger className="flex items-center gap-1 cursor-help">
                  Total Withdrawals
                </TooltipTrigger>
                <TooltipContent>
                  <p>Total funds withdrawn</p>
                </TooltipContent>
              </Tooltip>
            </CardTitle>
            <ArrowUpRight className="h-4 w-4 text-red-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {loading
                ? "..."
                : `${currencySymbol} ${summary?.total_withdraw?.toFixed(2) ?? "0.00"}`}
            </div>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              <Tooltip>
                <TooltipTrigger className="flex items-center gap-1 cursor-help">
                  Total Invested
                </TooltipTrigger>
                <TooltipContent>
                  <p>Net investment (Deposits - Withdrawals)</p>
                </TooltipContent>
              </Tooltip>
            </CardTitle>
            <Wallet className="h-4 w-4 text-blue-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {loading ? "..." : `${currencySymbol} ${totalInvested.toFixed(2)}`}
            </div>
          </CardContent>
        </Card>
      </div>
    </TooltipProvider>
  );
}
