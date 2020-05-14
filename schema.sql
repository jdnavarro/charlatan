CREATE TABLE podcast (
  uri TEXT PRIMARY KEY NOT NULL,
  title TEXT NOT NULL
);

CREATE TABLE episode (
  id TEXT PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  uri TEXT NOT NULL,
  podcast TEXT NOT NULL,
  FOREIGN KEY (podcast) REFERENCES podcast (uri)
  )
