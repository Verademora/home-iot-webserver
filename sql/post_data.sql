INSERT INTO testing.data(temperature, humidity)
VALUES ($1, $2)
RETURNING $table_fields;
