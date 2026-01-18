ALTER TABLE fiat_exchange_rate
ADD COLUMN is_non_working_day BOOLEAN DEFAULT 0;

ALTER TABLE fiat_exchange_rate
ADD COLUMN non_working_day_reason TEXT;
