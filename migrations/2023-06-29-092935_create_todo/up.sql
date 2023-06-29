-- Your SQL goes here
CREATE TABLE todos (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  content TEXT NOT NULL,
  completed BOOLEAN DEFAULT FALSE NOT NULL,
  when_will_it_be_done DATE NOT NULL
);
