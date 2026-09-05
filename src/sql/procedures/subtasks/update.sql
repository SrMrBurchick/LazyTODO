UPDATE Tasks
SET title = :newTitle,
    description = :newDescription
WHERE id = :targetId;
