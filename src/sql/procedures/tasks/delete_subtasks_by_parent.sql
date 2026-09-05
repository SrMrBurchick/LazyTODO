DELETE FROM SubTasks
WHERE parentTaskId = targetId
VALUES (
    :targetId
);
