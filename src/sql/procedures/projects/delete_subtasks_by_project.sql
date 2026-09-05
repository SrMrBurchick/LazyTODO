DELETE FROM SubTasks
WHERE parentTaskId IN (
    SELECT id
    FROM Tasks
    WHERE projectId = :targetId
);
