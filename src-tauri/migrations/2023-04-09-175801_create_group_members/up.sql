CREATE TABLE group_members(
    guest_id INT NOT NULL,
    group_id INT NOT NULL,
    is_leader BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (guest_id) REFERENCES guests(id),
    FOREIGN KEY (group_id) REFERENCES groupz(id),
    PRIMARY KEY(guest_id, group_id)
)
