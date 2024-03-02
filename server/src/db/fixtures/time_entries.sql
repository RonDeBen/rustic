
INSERT INTO time_entries (start_time, total_time, note, day, created_at)
VALUES
    (NULL, 3600000, "foo", 0, CURRENT_TIMESTAMP), -- Monday
    (CURRENT_TIMESTAMP, 0, "bar", 0, CURRENT_TIMESTAMP), -- Monday with active timer
    (NULL, 3600000, "baz", 1, CURRENT_TIMESTAMP); -- Tuesday
