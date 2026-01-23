--! This table is to ensure the secret of each exchanger can be stored propertly
CREATE TABLE IF NOT EXISTS crypto_exchanger (
    id TEXT PRIMARY KEY NOT NULL,
    api_key_encrypted BLOB NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER IF NOT EXISTS trg_crypto_exchanger_updated_at
AFTER UPDATE ON crypto_exchanger
FOR EACH ROW
WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE crypto_exchanger
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;
