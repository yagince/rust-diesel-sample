CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

ALTER TABLE posts ADD COLUMN user_id SERIAL;
ALTER TABLE posts ADD FOREIGN KEY (user_id) references users(id);
