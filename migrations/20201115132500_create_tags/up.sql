CREATE TABLE tags (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE UNIQUE INDEX tag_name ON tags (name);
CREATE UNIQUE INDEX tag_id ON tags (id);

CREATE TABEL post_tags (
  id SERIAL PRIMARY KEY,
  post_id SERIAL REFERENCES posts(id),
  tag_id SERIAL REFERENCES tags(id)
);

CREATE UNIQUE INDEX post_tags_post_id ON post_tags (post_id);
CREATE UNIQUE INDEX post_tags_tag_id ON post_tags (tag_id);

ALTER TABLE posts ADD tags VARCHAR[] DEFAULT ARRAY[];
