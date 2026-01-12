import { RowId, StringRowId } from "./common";

export type RampKind = "deposit" | "withdraw";

export interface CreateFiatRamp {
    fiat_id: RowId;
    fiat_amount: number;
    ramp_date: Date;
    via_exchange: string;
    kind: RampKind;
}

export interface FiatRamp {
    id: StringRowId;
    fiat_id: RowId;
    fiat_amount: number;
    ramp_date: Date;
    via_exchange: string;
    kind: RampKind;
    created_at: Date;
    updated_at: Date;
    fiat_symbol: string;
}

// must have id
export type UpdateFiatRamp = Partial<FiatRamp> & { id: StringRowId };

export interface FiatRampPagination {
    total_count: number;
    fiat_ramps: FiatRamp[];
}