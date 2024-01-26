-- Your SQL goes here
CREATE TABLE hodies (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  price NUMERIC(10,2) NOT NULL,
  image_url VARCHAR(255),
  brand VARCHAR(255),
  category VARCHAR(255)
);