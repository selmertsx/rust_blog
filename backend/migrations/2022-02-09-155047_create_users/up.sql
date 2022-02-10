CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL UNIQUE
);

CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  user_id integer NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT
);

CREATE INDEX posts_user_id ON posts (user_id);
