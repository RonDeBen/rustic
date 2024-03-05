
CREATE TABLE IF NOT EXISTS charge_codes (
    id INTEGER PRIMARY KEY,
    alias TEXT,
    code TEXT,
    is_nc BOOLEAN
);

CREATE TABLE IF NOT EXISTS time_entries (
    id INTEGER PRIMARY KEY,
    start_time TIMESTAMP NULL,
    total_time BIGINT,
    note TEXT,
    day SMALLINT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    charge_code_id INTEGER NULL REFERENCES charge_codes(id) ON DELETE SET NULL
);

-- if you ever want time_entries to have many charge codes
--CREATE TABLE time_tracking.time_entry_charge_code (
--    time_entry_id INTEGER REFERENCES time_tracking.time_entries(id) ON DELETE CASCADE,
--    charge_code_id INTEGER REFERENCES time_tracking.charge_codes(id) ON DELETE CASCADE,
--    PRIMARY KEY (time_entry_id, charge_code_id)
--);
