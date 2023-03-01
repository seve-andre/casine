CREATE TABLE documents(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    leader_id INT NOT NULL,
    doc_type TEXT NOT NULL,
    doc_number TEXT NOT NULL,
    birthplace TEXT NOT NULL,
    released_by TEXT NOT NULL,
    residence TEXT NOT NULL,
    FOREIGN KEY (leader_id) REFERENCES guests(id),
    UNIQUE(leader_id, id, doc_type, doc_number)
)
