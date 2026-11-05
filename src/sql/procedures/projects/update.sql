UPDATE Projects
SET title = :newTitle,
    description = :newDescription
WHERE id = :targetId;
