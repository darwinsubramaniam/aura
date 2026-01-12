CREATE TABLE IF NOT EXISTS fiat_ramp (
    id TEXT PRIMARY KEY,
    fiat_id INTEGER NOT NULL,
    fiat_amount REAL NOT NULL,
    ramp_date DATE NOT NULL DEFAULT CURRENT_DATE,
    via_exchange VARCHAR(255) NOT NULL,
    kind VARCHAR(8) NOT NULL CHECK (
        kind IN ('deposit', 'withdraw')
    ),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create index on fiat_id
CREATE INDEX idx_fiat_ramp_fiat_id ON fiat_ramp (fiat_id);

-- Create index on ramp_date
CREATE INDEX idx_fiat_ramp_ramp_date ON fiat_ramp (ramp_date);

-- Create index on via_exchange
CREATE INDEX idx_fiat_ramp_via_exchange ON fiat_ramp (via_exchange);

-- Create index on kind
CREATE INDEX idx_fiat_ramp_kind ON fiat_ramp (kind);

-- Update updated_at column on insert and update
CREATE TRIGGER update_fiat_ramp_updated_at
    BEFORE UPDATE ON fiat_ramp
    FOR EACH ROW
    BEGIN
        UPDATE fiat_ramp SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;