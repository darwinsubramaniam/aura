import { invoke } from "@tauri-apps/api/core"
import { Fiat } from "../../models/fiat"

enum FiatCommandList {
    GET_ALL_CURRENCIES = 'get_all_currencies',
    GET_CURRENCIES_BY_SYMBOL = 'get_currencies_by_symbol'
}

export class FiatCommand {
    public static getAllCurrencies() {
        return invoke<Fiat[]>(FiatCommandList.GET_ALL_CURRENCIES)
    }

    public static getCurrencyBySymbol(symbol: string) {
        return invoke<Fiat>(FiatCommandList.GET_CURRENCIES_BY_SYMBOL, { symbol })
    }
}
