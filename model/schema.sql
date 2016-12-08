CREATE TABLE users(
        id   INTEGER PRIMARY KEY,
				twitter_id INTEGER NOT NULL UNIQUE,
				twitter_screenname TEXT NOT NULL,
				twitter_name TEXT NOT NULL
        );
