CREATE TABLE fiat_exchange_rate (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    base_fiat_id INTEGER NOT NULL,
    date DATE NOT NULL,
    rates TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (base_fiat_id) REFERENCES fiat (id),
    UNIQUE (base_fiat_id, date)
);

-- create index on the base_fiat_id
CREATE INDEX idx_fiat_exchange_rate_base_fiat_id ON fiat_exchange_rate (base_fiat_id);

-- create index on the date
CREATE INDEX idx_fiat_exchange_rate_date ON fiat_exchange_rate (date);

-- Update updated_at column on insert and update
CREATE TRIGGER update_fiat_exchange_rate_updated_at
    BEFORE UPDATE ON fiat_exchange_rate
    FOR EACH ROW
    BEGIN
        UPDATE fiat_exchange_rate SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;