DROP VIEW IF EXISTS fiat_ramp_view;

CREATE VIEW IF NOT EXISTS fiat_ramp_view AS
SELECT
    t2.id as fiat_ramp_id,
    t2.fiat_id as `from_fiat_id`,
    t2.fiat_symbol as `from_fiat_symbol`,
    t2.fiat_name as `from_fiat_name`,
    t2.target_fiat_id as `to_fiat_id`,
    t2.target_fiat_symbol as `to_fiat_symbol`,
    t2.target_fiat_name as `to_fiat_name`,
    t2.to_rate / t2.from_rate as conversion_rate,
    t2.ramp_date as `ramp_date`,
    t2.fiat_amount as `fiat_amount`,
    t2.kind as `kind`,
    t2.via_exchange as `via_exchange`,
    COALESCE(t2.is_estimated, 0) as `is_estimated`,
    COALESCE(t2.is_non_working_day, 0) as `is_non_working_day`,
    t2.non_working_day_reason as `non_working_day_reason`,
    ROUND(
        t2.fiat_amount * (t2.to_rate / t2.from_rate),
        2
    ) as converted_amount
FROM (
        SELECT
            t1.*, CASE
                WHEN t1.fiat_symbol = t1.target_fiat_symbol THEN 1.0
                ELSE json_extract(
                    t1.rates, '$.' || t1.target_fiat_symbol
                )
            END as to_rate
        FROM (
                SELECT
                    fiat_ramp.id,
                    fiat_ramp.fiat_id,
                    fiat_ramp.fiat_amount,
                    fiat_ramp.ramp_date,
                    fiat_ramp.kind,
                    fiat_ramp.via_exchange,
                    fiat_ramp.created_at,
                    fiat_ramp.updated_at,
                    fiat.symbol as fiat_symbol,
                    fiat.name as fiat_name,
                    user_settings.default_fiat_id as target_fiat_id,
                    default_fiat.symbol as target_fiat_symbol,
                    default_fiat.name as target_fiat_name,
                    CASE
                        WHEN fiat_ramp.fiat_id = user_settings.default_fiat_id THEN 1.0
                        ELSE json_extract(
                            fiat_exchange_rate.rates, '$.' || fiat.symbol
                        )
                    END as from_rate,
                    fiat_exchange_rate.rates,
                    fiat_exchange_rate.is_estimated,
                    fiat_exchange_rate.is_non_working_day,
                    fiat_exchange_rate.non_working_day_reason
                FROM
                    fiat_ramp
                    JOIN fiat ON fiat.id = fiat_id
                    LEFT JOIN fiat_exchange_rate ON fiat_exchange_rate.date = ramp_date
                    JOIN fiat as default_fiat ON default_fiat.id = user_settings.default_fiat_id
                    JOIN user_settings ON user_settings.id = 1
            ) as t1
    ) as t2
ORDER BY t2.ramp_date ASC;
