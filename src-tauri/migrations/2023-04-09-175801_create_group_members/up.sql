CREATE TABLE group_members(
    guest_id INTEGER NOT NULL,
    group_id INTEGER NOT NULL,
    is_leader BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY(guest_id, group_id),
    FOREIGN KEY (guest_id) REFERENCES guests(id),
    FOREIGN KEY (group_id) REFERENCES groupz(id)
)
