import { render, screen, waitFor } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import FundingTable from "./funding-table";

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(() => Promise.resolve([])),
}));

// Mock the FiatRampCommand
vi.mock("@/lib/services/funding/fiatRamp.command", () => ({
  FiatRampCommand: {
    get: vi.fn(() =>
      Promise.resolve({
        fiat_ramps: [],
        total_count: 0,
      })
    ),
  },
}));

// Mock ResizeObserver
global.ResizeObserver = class ResizeObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
};

describe("FundingTable", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("displays 'Showing all records' when no date range is provided", async () => {
    render(<FundingTable />);
    
    // Wait for the data to load (even if empty)
    await waitFor(() => {
        const filterText = screen.getByTestId("date-range-filter-text");
        expect(filterText.textContent).toBe("Showing all records");
    });
  });

  it("displays formatted date range when start and end dates are provided", async () => {
    const startDate = new Date("2023-01-01");
    const endDate = new Date("2023-12-31");

    render(<FundingTable startDate={startDate} endDate={endDate} />);

    await waitFor(() => {
      const filterText = screen.getByTestId("date-range-filter-text");
      expect(filterText.textContent).toBe(
        "Showing records from Jan 1, 2023 to Dec 31, 2023"
      );
    });
  });
});
