import { format, subWeeks, subMonths, subYears, startOfDay, endOfDay } from "date-fns"
import { Calendar as CalendarIcon } from "lucide-react"
import { DateRange } from "react-day-picker"

import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"
import { Calendar } from "@/components/ui/calendar"
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

interface DateRangeFilterProps {
  date: DateRange | undefined
  setDate: (date: DateRange | undefined) => void
  className?: string
}

export function DateRangeFilter({
  date,
  setDate,
  className,
}: DateRangeFilterProps) {
  const handlePresetChange = (value: string) => {
    const end = endOfDay(new Date())
    let start: Date | undefined

    switch (value) {
      case "1week":
        start = startOfDay(subWeeks(end, 1))
        break
      case "2weeks":
        start = startOfDay(subWeeks(end, 2))
        break
      case "1month":
        start = startOfDay(subMonths(end, 1))
        break
      case "3months":
        start = startOfDay(subMonths(end, 3))
        break
      case "6months":
        start = startOfDay(subMonths(end, 6))
        break
      case "1year":
        start = startOfDay(subYears(end, 1))
        break
      case "all":
        setDate(undefined)
        return
    }

    if (start) {
      setDate({ from: start, to: end })
    }
  }

  return (
    <div className={cn("flex flex-col sm:flex-row items-start sm:items-center gap-2 w-full md:w-auto", className)}>
      <Popover>
        <PopoverTrigger asChild>
          <Button
            id="date"
            variant={"outline"}
            className={cn(
              "w-full sm:w-[300px] justify-start text-left font-normal",
              !date && "text-muted-foreground"
            )}
          >
            <CalendarIcon className="mr-2 h-4 w-4" />
            {date?.from ? (
              date.to ? (
                <>
                  {format(date.from, "LLL dd, y")} -{" "}
                  {format(date.to, "LLL dd, y")}
                </>
              ) : (
                format(date.from, "LLL dd, y")
              )
            ) : (
              <span>Pick a date range</span>
            )}
          </Button>
        </PopoverTrigger>
        <PopoverContent className="w-auto p-0" align="start">
          <Calendar
            initialFocus
            mode="range"
            defaultMonth={date?.from}
            selected={date}
            onSelect={setDate}
            numberOfMonths={2}
          />
        </PopoverContent>
      </Popover>
      <Select onValueChange={handlePresetChange}>
        <SelectTrigger className="w-full sm:w-[120px]">
          <SelectValue placeholder="Presets" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="1week">1 Week</SelectItem>
          <SelectItem value="2weeks">2 Weeks</SelectItem>
          <SelectItem value="1month">1 Month</SelectItem>
          <SelectItem value="3months">3 Months</SelectItem>
          <SelectItem value="6months">6 Months</SelectItem>
          <SelectItem value="1year">1 Year</SelectItem>
          <SelectItem value="all">All Time</SelectItem>
        </SelectContent>
      </Select>
    </div>
  )
}
