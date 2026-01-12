import { StringRowId } from "@/lib/models/common";
import { CreateFiatRamp, FiatRamp, FiatRampPagination, UpdateFiatRamp } from "@/lib/models/fiatRamp";
import { invoke } from "@tauri-apps/api/core";
import { format } from "date-fns";

enum FiatRampCommandList {
    CREATE = 'create_fiat_ramp',
    GET = 'get_fiat_ramps',
    UPDATE = 'update_fiat_ramp',
    DELETE = 'delete_fiat_ramp'
}


export class FiatRampCommand {

    /**
     * Create a new fiat ramp
     * @param createFiatRamp 
     * @returns String = id of the newly created fiat ramp
     */
    public static create(createFiatRamp: CreateFiatRamp) {
        return invoke<StringRowId>(FiatRampCommandList.CREATE, {
            createFiatRamp: {
                fiat_id: createFiatRamp.fiat_id,
                fiat_amount: createFiatRamp.fiat_amount,
                ramp_date: format(createFiatRamp.ramp_date, 'yyyy-MM-dd'),
                via_exchange: createFiatRamp.via_exchange,
                kind: createFiatRamp.kind
            }
        });
    }

    /**
     * Get all fiat ramps
     * @param limit Optional limit for pagination
     * @param offset Optional offset for pagination
     * @param query Optional query for search
     * @returns FiatRampPagination
     */
    public static get(limit?: number, offset?: number, query?: string) {
        return invoke<FiatRampPagination>(FiatRampCommandList.GET, { limit, offset, query });
    }

    /**
     * Update the fiat ramp
     * @param fiatRamp The fiat ramp to update
     * @returns number = number of rows affected
     */
    public static update(fiatRamp: UpdateFiatRamp) {
        return invoke<number>(FiatRampCommandList.UPDATE, {
            fiatRamp: {
                id: fiatRamp.id,
                fiat_id: fiatRamp.fiat_id,
                fiat_amount: fiatRamp.fiat_amount,
                ramp_date: fiatRamp.ramp_date ? format(fiatRamp.ramp_date, 'yyyy-MM-dd') : undefined,
                via_exchange: fiatRamp.via_exchange,
                kind: fiatRamp.kind
            }
        });
    }

    /**
     * Delete the fiat ramp
     * @param id 
     * @returns number = number of rows affected
     */
    public static delete(id: StringRowId) {
        return invoke<number>(FiatRampCommandList.DELETE, { id });
    }
}