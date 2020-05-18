CREATE TABLE podcast (
  src TEXT PRIMARY KEY NOT NULL,
  title TEXT NOT NULL
);

CREATE TABLE episode (
  id INTEGER PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  src TEXT NOT NULL,
  progress INTEGER NOT NULL,
  podcast TEXT NOT NULL,
  FOREIGN KEY (podcast) REFERENCES podcast (src)
  );
