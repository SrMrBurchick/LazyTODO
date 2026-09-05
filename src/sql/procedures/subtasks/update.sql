UPDATE SubTasks
SET title = newTitle,
    description = newDescription
WHERE id = targetId
VALUES(
    :targetId,
    :newTitle,
    :newDescription
)
