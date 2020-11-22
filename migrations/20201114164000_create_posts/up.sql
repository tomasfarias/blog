CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  slug VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  published_at TIMESTAMP NULL,
  user_id INT,
  CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE UNIQUE INDEX slug_idx ON posts (slug);
