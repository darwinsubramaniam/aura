export interface FiatRate {
    id: number;
    base_fiat_id: number;
    date: string;
    rate: [string, number][];
}