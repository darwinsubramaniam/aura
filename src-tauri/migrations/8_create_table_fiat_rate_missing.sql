CREATE TABLE fiat_rate_missing (
    fiat_ramp_id TEXT PRIMARY KEY,
    base_fiat_id INTEGER NOT NULL,
    date DATE NOT NULL,
    error_count INTEGER DEFAULT 0,
    last_error_msg TEXT,
    last_attempt_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (fiat_ramp_id) REFERENCES fiat_ramp (id) ON DELETE CASCADE,
    FOREIGN KEY (base_fiat_id) REFERENCES fiat (id)
);

CREATE TRIGGER update_fiat_rate_missing_updated_at
    BEFORE UPDATE ON fiat_rate_missing
    FOR EACH ROW
    BEGIN
        UPDATE fiat_rate_missing SET updated_at = CURRENT_TIMESTAMP WHERE fiat_ramp_id = NEW.fiat_ramp_id;
    END;

CREATE INDEX idx_fiat_rate_missing_base_date ON fiat_rate_missing (base_fiat_id, date);