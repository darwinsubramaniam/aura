CREATE TABLE IF NOT EXISTS user_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    locale VARCHAR(255) NOT NULL,
    default_fiat_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (default_fiat_id) REFERENCES fiat (id)
);

-- Auto update the updated_at column whenever the row is updated
CREATE TRIGGER IF NOT EXISTS update_user_settings_updated_at
    BEFORE UPDATE ON user_settings
    FOR EACH ROW
    BEGIN
        UPDATE user_settings SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;