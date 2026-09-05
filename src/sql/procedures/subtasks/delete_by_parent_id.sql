DELETE FROM SubTasks
WHERE parentTaskId = :targetId;
