CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  slug VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  created_at TIMESTAMP NOT NULL,
  published_at TIMESTAMP NULL
);

CREATE UNIQUE INDEX slug_idx ON posts (slug);
