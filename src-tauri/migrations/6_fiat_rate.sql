CREATE TABLE fiat_rate (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    base_fiat_id INTEGER NOT NULL,
    date DATE NOT NULL,
    rates TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (base_fiat_id) REFERENCES fiat (id),
    UNIQUE (base_fiat_id, date)
);