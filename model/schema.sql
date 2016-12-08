PRAGMA foreign_kes=ON;
CREATE TABLE users(
	id   INTEGER PRIMARY KEY,
	twitter_id INTEGER NOT NULL UNIQUE,
	twitter_screenname TEXT NOT NULL,
	twitter_name TEXT NOT NULL
);

CREATE TABLE papers(
	id   INTEGER PRIMARY KEY,
	author_id INTEGER,
	title TEXT NOT NULL,
	abst_url TEXT,
	cmt TEXT,
	FOREIGN KEY(author_id) REFERENCES users(id)
);

CREATE TABLE comments(
	id   INTEGER PRIMARY KEY,
	user_id INTEGER,
	cmt TEXT,
	FOREIGN KEY(user_id) REFERENCES users(id)
);
