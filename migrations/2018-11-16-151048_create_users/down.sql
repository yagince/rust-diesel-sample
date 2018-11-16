ALTER TABLE posts DELETE FOREIGN KEY (user_id) references users(id);
ALTER TABLE posts DELETE COLUMN user_id SERIAL;
DROP TABLE users;
