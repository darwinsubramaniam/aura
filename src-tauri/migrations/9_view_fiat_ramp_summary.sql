DROP VIEW IF EXISTS fiat_ramp_summary;

CREATE VIEW IF NOT EXISTS fiat_ramp_summary AS
SELECT
    SUM(converted_amount) as total_conversions,
    kind,
    to_fiat_name,
    to_fiat_symbol
FROM fiat_ramp_view
GROUP BY
    kind;