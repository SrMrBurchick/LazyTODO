INSERT INTO SubTasks (
    title,
    parentTaskId,
    state,
    description
)
VALUES (
    :title,
    :parentTaskId,
    :state,
    :description
);
