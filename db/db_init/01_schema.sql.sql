CREATE schema IF NOT EXISTS time_tracking;

CREATE TABLE IF NOT EXISTS time_tracking.charge_codes (
    id SERIAL PRIMARY KEY,
    alias TEXT,
    code TEXT,
    is_nc BOOLEAN
);

CREATE TABLE IF NOT EXISTS time_tracking.time_entries (
    id SERIAL PRIMARY KEY,
    start_time TIMESTAMP NULL,
    total_time BIGINT,
    note TEXT,
    day SMALLINT,
    charge_code_id INTEGER NULL REFERENCES time_tracking.charge_codes(id) ON DELETE SET NULL
);

CREATE TABLE time_tracking.time_entry_charge_code (
    time_entry_id INTEGER REFERENCES time_tracking.time_entries(id) ON DELETE CASCADE,
    charge_code_id INTEGER REFERENCES time_tracking.charge_codes(id) ON DELETE CASCADE,
    PRIMARY KEY (time_entry_id, charge_code_id)
);
