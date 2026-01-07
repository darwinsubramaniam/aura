export interface Fiat {
    id: number;
    symbol: string;
    name: string;
}

export interface FiatRamp {
    id: string;
    fiat_id: number;
    fiat_amount: number;
    date: string;
    via_exchange: string;
    kind: string;
    created_at: string;
    updated_at: string;
}

export interface FiatRampPagination {
    total_count: number;
    fiat_ramps: FiatRamp[];
}
