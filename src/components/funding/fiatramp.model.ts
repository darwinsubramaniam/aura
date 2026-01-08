export interface Fiat {
    id: number;
    symbol: string;
    name: string;
}

export interface Funding {
    id: string;
    fiat_id: number;
    fiat_amount: number;
    ramp_date: string;
    via_exchange: string;
    kind: string;
    created_at: string;
    updated_at: string;
}

export interface FundingPagination {
    total_count: number;
    fiat_ramps: Funding[];
}
