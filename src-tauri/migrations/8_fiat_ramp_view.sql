-- Create a view that joins fiat_ramp with fiat_rate to show conversion details
CREATE VIEW IF NOT EXISTS fiat_ramp_view AS
SELECT
    t2.*,
    t2.to_rate / t2.from_rate as conversion_rate,
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
                    fiat_ramp.id, fiat_ramp.fiat_id, fiat_ramp.fiat_amount, fiat_ramp.ramp_date, fiat_ramp.kind, fiat_ramp.via_exchange, fiat_ramp.created_at, fiat_ramp.updated_at, fiat.symbol as fiat_symbol, user_settings.default_fiat_id as target_fiat_id, default_fiat.symbol as target_fiat_symbol, CASE
                        WHEN fiat_ramp.fiat_id = user_settings.default_fiat_id THEN 1.0
                        ELSE json_extract(
                            fiat_rate.rates, '$.' || fiat.symbol
                        )
                    END as from_rate, fiat_rate.rates
                FROM
                    fiat_ramp
                    JOIN fiat ON fiat.id = fiat_id
                    JOIN fiat_rate ON fiat_rate.date = ramp_date
                    JOIN fiat as default_fiat ON default_fiat.id = user_settings.default_fiat_id
                    JOIN user_settings ON user_settings.id = 1
            ) as t1
    ) as t2