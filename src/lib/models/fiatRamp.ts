import { RowId, StringRowId } from "./common";

export type RampKind = "deposit" | "withdraw";

export type SortDirection = "asc" | "desc";

export interface SortOptions {
  column?: string;
  direction?: SortDirection;
}

export interface CreateFiatRamp {
  fiat_id: RowId;
  fiat_amount: number;
  ramp_date: Date;
  via_exchange: string;
  kind: RampKind;
}

export interface FiatRampView {
  fiat_ramp_id: StringRowId;
  from_fiat_id: RowId;
  from_fiat_symbol: string;
  from_fiat_name: string;
  to_fiat_id: RowId;
  to_fiat_symbol: string;
  to_fiat_name: string;
  conversion_rate: number | null;
  ramp_date: string;
  fiat_amount: number;
  kind: RampKind;
  via_exchange: string;
  is_estimated: boolean;
  is_non_working_day: boolean;
  non_working_day_reason: string | null;
  converted_amount: number | null;
}

// must have id
export interface UpdateFiatRamp {
  id: StringRowId;
  fiat_id?: RowId;
  fiat_amount?: number;
  ramp_date?: Date;
  via_exchange?: string;
  kind?: RampKind;
}

export interface FiatRampPagination {
  total_count: number;
  fiat_ramps: FiatRampView[];
}

export interface FiatRampSummary {
  total_deposit: number;
  total_withdraw: number;
  fiat_symbol: string;
  fiat_name: string;
}
