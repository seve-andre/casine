CREATE TABLE documents(
    id INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    leader_id INT NOT NULL,
    doc_type VARCHAR(200) NOT NULL,
    doc_number VARCHAR(200) NOT NULL,
    birthplace TEXT NOT NULL,
    released_by TEXT NOT NULL,
    residence TEXT NOT NULL,
    FOREIGN KEY (leader_id) REFERENCES guests(id),
    UNIQUE(leader_id, id, doc_type, doc_number)
)
