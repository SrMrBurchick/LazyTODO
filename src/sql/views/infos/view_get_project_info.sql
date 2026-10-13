WITH task_stats AS (
    SELECT
        COUNT(*) AS total,
        COALESCE(
            SUM(CASE WHEN t.state = 2 THEN 1 ELSE 0 END),
            0
        ) AS completed
    FROM v_GetProjectsTasks t
    WHERE t.projectId = :projectId
),
subtask_stats AS (
    SELECT
        COUNT(*) AS total,
        COALESCE(
            SUM(CASE WHEN st.state = 2 THEN 1 ELSE 0 END),
            0
        ) AS completed
    FROM v_GetSubTasks st
    JOIN Tasks t ON t.id = st.parentTaskId
    WHERE t.projectId = :projectId
),
active_tasks AS (
    SELECT
        COALESCE(
            json_group_array(
                json_object(
                    'id', t.id,
                    'title', t.title,
                    'description', t.description,
                    'state', t.state
                )
            ),
            json('[]')
        ) AS items
    FROM v_GetProjectsTasks t
    WHERE
        t.projectId = :projectId
        AND t.state = 1
)
SELECT
    p.id,
    p.title,
    p.description,

    task_stats.total + subtask_stats.total AS total,

    task_stats.completed + subtask_stats.completed AS completed,

    CASE
        WHEN task_stats.total + subtask_stats.total = 0 THEN 0
        ELSE ROUND(
            100.0 *
            (task_stats.completed + subtask_stats.completed) /
            (task_stats.total + subtask_stats.total)
        )
    END AS progress,

    active_tasks.items AS activeTasks

FROM Projects p
CROSS JOIN task_stats
CROSS JOIN subtask_stats
CROSS JOIN active_tasks
WHERE p.id = :projectId;
