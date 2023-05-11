CREATE TABLE groupz(
    id INTEGER NOT NULL,
    nickname VARCHAR(100) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE(nickname)
)
