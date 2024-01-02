CREATE schema IF NOT EXISTS time_tracking;

CREATE TABLE time_tracking.time_entries (
    id SERIAL PRIMARY KEY,
    start_time TIMESTAMP,
    total_time DOUBLE PRECISION,
    note TEXT,
    day SMALLINT
);

CREATE TABLE time_tracking.charge_codes (
    id SERIAL PRIMARY KEY,
    alias TEXT,
    code TEXT,
    is_nc BOOLEAN
);

CREATE TABLE time_tracking.time_entry_charge_code (
    time_entry_id INTEGER REFERENCES time_tracking.time_entries(id) ON DELETE CASCADE,
    charge_code_id INTEGER REFERENCES time_tracking.charge_codes(id) ON DELETE CASCADE,
    PRIMARY KEY (time_entry_id, charge_code_id)
);
