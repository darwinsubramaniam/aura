import { beforeAll, describe, it, expect } from "vitest";
import { mockIPC, clearMocks } from "@tauri-apps/api/mocks";
import { FiatCommand } from "./fiat.command";
import { Fiat } from "../../models/fiat";

describe('FiatCommand', () => {
    beforeAll(() => {
        // Polyfill crypto for mockIPC
        Object.defineProperty(window, 'crypto', {
            value: {
                getRandomValues: (buffer: any) => {
                    return require('crypto').randomFillSync(buffer);
                },
            },
        });
    });

    const mockFiat: Fiat = {
        id: 1,
        symbol: 'USD',
        name: 'US Dollar',
        created_at: new Date(),
        updated_at: new Date(),
    };

    it('should get all fiat', async () => {
        mockIPC((cmd) => {
            if (cmd === 'get_all_currencies') {
                return [mockFiat];
            }
        });

        const fiat = await FiatCommand.getAllCurrencies()
        expect(fiat).toEqual([mockFiat]);
        clearMocks();
    });

    it('should get fiat by symbol', async () => {
        mockIPC((cmd, args) => {
            if (cmd === 'get_currencies_by_symbol') {
                const payload = args as { symbol: string };
                if (payload?.symbol === 'USD') {
                    return mockFiat;
                }
            }
        });

        const fiat = await FiatCommand.getCurrencyBySymbol('USD');
        expect(fiat).toEqual(mockFiat);
        clearMocks();
    });
});