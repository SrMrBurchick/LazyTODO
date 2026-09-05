UPDATE Tasks
SET state = :newState
WHERE id = :targetId;
