import { FiatRate } from "@/lib/models/fiat-rate";
import { invoke } from "@tauri-apps/api/core";

export class FiatRateService {
    public static async getRate(base_fiat_id: number, date: string) {
        return invoke("get_fiat_rate", { base_fiat_id, date }) as Promise<FiatRate>;
    }
}