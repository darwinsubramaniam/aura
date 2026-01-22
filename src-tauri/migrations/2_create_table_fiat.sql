CREATE TABLE IF NOT EXISTS fiat (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    symbol TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create index on symbol
CREATE INDEX idx_fiat_symbol ON fiat (symbol);

-- Update updated_at column on insert and update
CREATE TRIGGER update_fiat_updated_at
    BEFORE UPDATE ON fiat
    FOR EACH ROW
    BEGIN
        UPDATE fiat SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;