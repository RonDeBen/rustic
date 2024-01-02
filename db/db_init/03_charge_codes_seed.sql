COPY time_tracking.charge_codes (
    alias,
    code,
    is_nc
)
FROM '/docker-entrypoint-initdb.d/charge_codes.csv'
DELIMITER ','
CSV HEADER;

