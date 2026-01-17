import { StringRowId } from "@/lib/models/common";
import { CreateFiatRamp, FiatRampPagination, UpdateFiatRamp, SortOptions, FiatRampSummary } from "@/lib/models/fiatRamp";
import { invoke } from "@tauri-apps/api/core";
import { format } from "date-fns";

enum FiatRampCommandList {
    CREATE = 'create_fiat_ramp',
    GET = 'get_fiat_ramps',
    UPDATE = 'update_fiat_ramp',
    DELETE = 'delete_fiat_ramp',
    CREATE_BULK = 'create_fiat_ramps_bulk',
    GET_SUMMARY = 'get_fiat_ramp_summary',
    GET_DATE_RANGE = 'get_fiat_ramp_date_range'
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
     * Create multiple fiat ramps
     * @param ramps List of ramps to create
     * @returns number = total created
     */
    public static createBulk(ramps: CreateFiatRamp[]) {
        const payload = ramps.map(r => ({
            fiat_id: r.fiat_id,
            fiat_amount: r.fiat_amount,
            ramp_date: format(r.ramp_date, 'yyyy-MM-dd'),
            via_exchange: r.via_exchange,
            kind: r.kind
        }));
        
        return invoke<number>(FiatRampCommandList.CREATE_BULK, {
            ramps: payload
        });
    }

    /**
     * Get all fiat ramps
     * @param limit Optional limit for pagination
     * @param offset Optional offset for pagination
     * @param query Optional query for search
     * @param sort Optional sorting options
     * @param startDate Optional start date filter
     * @param endDate Optional end date filter
     * @returns FiatRampPagination
     */
    public static get(limit?: number, offset?: number, query?: string, sort?: SortOptions, startDate?: Date | null, endDate?: Date | null) {
        return invoke<FiatRampPagination>(FiatRampCommandList.GET, { 
            limit, 
            offset, 
            query, 
            sort,
            startDate: startDate ? format(startDate, 'yyyy-MM-dd') : undefined,
            endDate: endDate ? format(endDate, 'yyyy-MM-dd') : undefined
        });
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

    /**
     * Get fiat ramp summary
     * @param startDate Optional start date filter
     * @param endDate Optional end date filter
     * @returns FiatRampSummary
     */
    public static getSummary(startDate?: Date | null, endDate?: Date | null) {
        return invoke<FiatRampSummary>(FiatRampCommandList.GET_SUMMARY, {
            startDate: startDate ? format(startDate, 'yyyy-MM-dd') : undefined,
            endDate: endDate ? format(endDate, 'yyyy-MM-dd') : undefined
        });
    }

    /**
     * Get the min and max date of all fiat ramps
     * @returns [string | null, string | null] (min_date, max_date)
     */
    public static getDateRange() {
        return invoke<[string | null, string | null]>(FiatRampCommandList.GET_DATE_RANGE);
    }
}
