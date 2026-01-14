import ReactECharts from "echarts-for-react";
import { useEffect, useState, useMemo } from "react";
import { FiatRampView } from "@/lib/models/fiatRamp";
import { FiatRampCommand } from "@/lib/services/funding/fiatRamp.command";
import { invoke } from "@tauri-apps/api/core";
import { Fiat } from "@/lib/models/fiat";
import { FiatCommand } from "@/lib/services/fiat/fiat.command";
import { Button } from "@/components/ui/button";
import { BarChart3, LineChart, Inbox } from "lucide-react";

interface UserSettings {
  default_fiat_id: number;
}

type ChartType = "bar" | "line";

interface FundingChartSummaryProps {
  refreshTrigger?: number;
}

export default function FundingChartSummary({ refreshTrigger }: FundingChartSummaryProps) {
  const [fiatRamps, setFiatRamps] = useState<FiatRampView[]>([]);
  const [targetCurrency, setTargetCurrency] = useState<Fiat | null>(null);
  const [chartType, setChartType] = useState<ChartType>("bar");
  const [isLoading, setIsLoading] = useState(true);

  const loadData = async () => {
    try {
      setIsLoading(true);
      // Load user settings to get target currency
      const settings = await invoke<UserSettings>("get_user_settings");
      const fiats = await FiatCommand.getAllCurrencies();
      const target = fiats.find((f) => f.id === settings.default_fiat_id);
      setTargetCurrency(target || null);

      // Load all fiat ramps (no pagination for chart)
      const result = await FiatRampCommand.get(1000, 0);
      setFiatRamps(result.fiat_ramps);
    } catch (error) {
      console.error("Failed to load chart data:", error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, [refreshTrigger]);

  // Aggregate converted amounts by date
  const chartData = useMemo(() => {
    const dateMap = new Map<string, number>();

    fiatRamps.forEach((ramp) => {
      const date = ramp.ramp_date;
      const amount = ramp.converted_amount ?? 0;
      const currentTotal = dateMap.get(date) ?? 0;
      
      // Add for deposits, subtract for withdrawals
      if (ramp.kind === "deposit") {
        dateMap.set(date, currentTotal + amount);
      } else {
        dateMap.set(date, currentTotal - amount);
      }
    });

    // Sort by date
    const sortedEntries = Array.from(dateMap.entries()).sort(
      (a, b) => new Date(a[0]).getTime() - new Date(b[0]).getTime()
    );

    return {
      dates: sortedEntries.map(([date]) => date),
      amounts: sortedEntries.map(([, amount]) => amount),
    };
  }, [fiatRamps]);

  const currencySymbol = targetCurrency?.symbol.toUpperCase() ?? "USD";

  const option = useMemo(() => {
    const baseOption = {
      title: {
        text: `Funding Summary (${currencySymbol})`,
        textStyle: {
          color: "#888",
        },
      },
      tooltip: {
        trigger: "axis" as const,
      },
      legend: {
        show: false,
      },
      xAxis: {
        type: "category" as const,
        data: chartData.dates,
        axisLabel: {
          rotate: 45,
        },
      },
      yAxis: {
        type: "value" as const,
        axisLabel: {
          formatter: `{value} ${currencySymbol}`,
        },
      },
      series: [
        {
          name: "Converted Amount",
          data: chartData.amounts,
          type: chartType,
          smooth: chartType === "line",
          itemStyle: {
            color: (params: { value: number }) => {
              return params.value >= 0 ? "#22c55e" : "#fb7185";
            },
          },
          ...(chartType === "line" ? {
            lineStyle: {
              width: 2,
              color: "#22c55e",
            },
            areaStyle: {
              color: {
                type: "linear",
                x: 0,
                y: 0,
                x2: 0,
                y2: 1,
                colorStops: [
                  { offset: 0, color: "rgba(34, 197, 94, 0.4)" },
                  { offset: 0.5, color: "rgba(34, 197, 94, 0.1)" },
                  { offset: 1, color: "rgba(251, 113, 133, 0.4)" },
                ],
              },
            },
          } : {}),
        },
      ],
    };

    return baseOption;
  }, [chartData, currencySymbol, chartType]);

  if (isLoading) {
    return (
      <div className="h-[400px] w-full flex items-center justify-center text-muted-foreground animate-pulse">
        Loading chart data...
      </div>
    );
  }

  if (fiatRamps.length === 0) {
    return (
      <div className="h-[400px] w-full flex flex-col items-center justify-center text-muted-foreground border rounded-lg bg-card/50 border-dashed">
        <Inbox className="h-10 w-10 mb-2 opacity-50" />
        <p className="text-lg font-medium">No Funding Activity</p>
        <p className="text-sm">There is no funding data available to display in the chart.</p>
      </div>
    );
  }

  return (
    <div>
      <div className="flex justify-end gap-1 mb-2">
        <Button
          variant={chartType === "bar" ? "default" : "outline"}
          size="sm"
          onClick={() => setChartType("bar")}
        >
          <BarChart3 className="h-4 w-4" />
        </Button>
        <Button
          variant={chartType === "line" ? "default" : "outline"}
          size="sm"
          onClick={() => setChartType("line")}
        >
          <LineChart className="h-4 w-4" />
        </Button>
      </div>
      <ReactECharts option={option} />
    </div>
  );
}
