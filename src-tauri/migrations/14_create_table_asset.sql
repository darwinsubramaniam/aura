CREATE TABLE IF NOT EXISTS asset (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    symbol TEXT NOT NULL UNIQUE,
    kind TEXT NOT NULL CHECK (kind IN ('stablecoin','cryptocoin','nft')),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Auto-update updated_at on updates (SQLite-compatible; avoids infinite loop)
CREATE TRIGGER IF NOT EXISTS trg_asset_updated_at
AFTER UPDATE ON asset
FOR EACH ROW
WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE asset
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;
