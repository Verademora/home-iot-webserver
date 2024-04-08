DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;
CREATE TABLE testing.data(
    id  BIGSERIAL PRIMARY KEY,
    temperature REAL,
    humidity    REAL
);
