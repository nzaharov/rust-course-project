CREATE TABLE entries (
    id SERIAL PRIMARY KEY,
    pc_name TEXT NOT NULL,
    cpu_usage TEXT NOT NULL,
    mem_usage TEXT NOT NULL,
    recorded_at BIGINT NOT NULL
)