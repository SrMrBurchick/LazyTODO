UPDATE SubTasks
SET state = :newState
WHERE id = :targetId;
