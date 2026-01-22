CREATE TABLE IF NOT EXISTS crypto_exchanger (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    base_url TEXT NOT NULL,
    data TEXT CHECK(data IS NULL OR json_valid(data)),
    api_key_encrypted BLOB NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_crypto_exchanger_name
ON crypto_exchanger(name);

CREATE TRIGGER IF NOT EXISTS trg_crypto_exchanger_updated_at
AFTER UPDATE ON crypto_exchanger
FOR EACH ROW
WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE crypto_exchanger
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;






