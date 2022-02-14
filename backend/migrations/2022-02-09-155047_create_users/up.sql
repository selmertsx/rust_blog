CREATE TABLE users (
  id VARCHAR(26) PRIMARY KEY,
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL UNIQUE
);

CREATE TABLE posts (
  id VARCHAR(26) PRIMARY KEY,
  user_id VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT
);

CREATE INDEX posts_user_id ON posts (user_id);
