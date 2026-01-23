CREATE TABLE IF NOT EXISTS asset_crypto_exchanger (
    asset_id INTEGER NOT NULL,
    crypto_exchanger_id INTEGER NOT NULL,
    actual_asset_id_in_exchanger TEXT NOT NULL, --! the id of the asset in the exchanger this can be number or string depend on the exchanger. conversion fall on the exchanger
    data TEXT NULL, --! json data to store exchanger unique data about the asset.
    priority INTEGER NOT NULL DEFAULT 100,
    enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0,1)),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (asset_id, crypto_exchanger_id),
    FOREIGN KEY (asset_id)
        REFERENCES asset(id)
        ON DELETE CASCADE,
    FOREIGN KEY (crypto_exchanger_id)
        REFERENCES crypto_exchanger(id)
        ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_ace_asset_priority
ON asset_crypto_exchanger(asset_id, enabled, priority, crypto_exchanger_id);

CREATE INDEX IF NOT EXISTS idx_ace_exchanger
ON asset_crypto_exchanger(crypto_exchanger_id);

CREATE TRIGGER IF NOT EXISTS trg_asset_crypto_exchanger_updated_at
AFTER UPDATE ON asset_crypto_exchanger
FOR EACH ROW
WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE asset_crypto_exchanger
    SET updated_at = CURRENT_TIMESTAMP
    WHERE asset_id = NEW.asset_id AND crypto_exchanger_id = NEW.crypto_exchanger_id;
END;
