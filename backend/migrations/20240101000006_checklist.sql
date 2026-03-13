CREATE TABLE IF NOT EXISTS checklist_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    label TEXT NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS checklist_checks (
    item_id INTEGER NOT NULL,
    editor_email TEXT NOT NULL,
    checked INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (item_id, editor_email),
    FOREIGN KEY (item_id) REFERENCES checklist_items(id) ON DELETE CASCADE
);
