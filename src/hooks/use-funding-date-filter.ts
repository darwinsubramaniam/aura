import { useState, useEffect } from "react";
import { DateRange } from "react-day-picker";
import { subWeeks, startOfDay, endOfDay } from "date-fns";

const STORAGE_KEY = "funding_date_filter_range";

export function useFundingDateFilter() {
  const [dateRange, setDateRange] = useState<DateRange | undefined>(() => {
    // 1. Try to load from localStorage
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      try {
        const parsed = JSON.parse(stored);
        if (parsed.from && parsed.to) {
          return {
            from: new Date(parsed.from),
            to: new Date(parsed.to),
          };
        }
      } catch (e) {
        console.error("Failed to parse stored date range", e);
      }
    }

    // 2. Default to last 2 weeks if nothing stored
    const end = endOfDay(new Date());
    const start = startOfDay(subWeeks(end, 2));
    return {
      from: start,
      to: end,
    };
  });

  // Update localStorage whenever dateRange changes
  useEffect(() => {
    if (dateRange?.from && dateRange?.to) {
      localStorage.setItem(
        STORAGE_KEY,
        JSON.stringify({
          from: dateRange.from.toISOString(),
          to: dateRange.to.toISOString(),
        })
      );
    } else {
      localStorage.removeItem(STORAGE_KEY);
    }
  }, [dateRange]);

  return { dateRange, setDateRange };
}
