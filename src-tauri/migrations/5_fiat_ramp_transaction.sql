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