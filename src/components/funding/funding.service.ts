import { invoke } from "@tauri-apps/api/core";
import { Fiat } from "./fiatramp.model";


export default class FundingService {
    public static async getAllFiats() {
        return await invoke<Fiat[]>('get_all_fiat');
    }

    public static async createFiatRamp(
        fiat_id: number,
        fiat_amount: number,
        fund_date: string,
        via_exchange: string,
        kind: string
    ) {
        await invoke('create_fiat_ramp', {
            createFiatRamp: {
                fiat_id: fiat_id,
                fiat_amount: fiat_amount,
                ramp_date: fund_date,
                via_exchange: via_exchange,
                kind: kind,
            }
        })
    }
}