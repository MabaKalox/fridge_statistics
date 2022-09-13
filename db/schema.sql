CREATE TABLE IF NOT EXISTS temps (
    id          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    sensor_id   INTEGER NOT NULL,
    temp        REAL NOT NULL,
    created     INTEGER NOT NULL
);
