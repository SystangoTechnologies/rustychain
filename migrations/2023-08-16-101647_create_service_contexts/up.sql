CREATE TABLE service_contexts (
  id SERIAL PRIMARY KEY,
  maintenance BOOLEAN NOT NULL DEFAULT FALSE
);