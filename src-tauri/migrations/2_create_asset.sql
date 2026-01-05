CREATE TABLE IF NOT EXISTS asset (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    symbol TEXT NOT NULL UNIQUE,
    kind TEXT NOT NULL CHECK (
        kind IN (
            'stablecoin',
            'cryptocoin',
            'nft'
        )
    ),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create index on symbol
CREATE INDEX idx_asset_symbol ON asset (symbol);