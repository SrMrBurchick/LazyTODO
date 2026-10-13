WITH subtask_stats AS (
    SELECT
        COUNT(*) AS total,
        COALESCE(
            SUM(
                CASE
                    WHEN st.state = 2 THEN 1
                    ELSE 0
                END
            ),
            0
        ) AS completed
    FROM SubTasks st
    WHERE st.parentTaskId = :taskId
),
active_subtasks AS (
    SELECT
        COALESCE(
            json_group_array(
                json_object(
                    'id', st.id,
                    'title', st.title,
                    'description', st.description,
                    'state', st.state
                )
            ),
            json('[]')
        ) AS items
    FROM v_GetSubTasks st
    WHERE
        st.parentTaskId = :taskId
        AND st.state = 1
)
SELECT
    t.id,
    t.title,
    t.description,
    t.state,
    t.projectId,
    p.title AS projectTitle,

    subtask_stats.total,
    subtask_stats.completed,

    CASE
        WHEN subtask_stats.total = 0 THEN
            CASE
                WHEN t.state = 2 THEN 100
                ELSE 0
            END
        ELSE ROUND(
            100.0 *
            subtask_stats.completed /
            subtask_stats.total
        )
    END AS progress,

    active_subtasks.items AS activeSubTasks

FROM Tasks t
CROSS JOIN subtask_stats
CROSS JOIN active_subtasks
LEFT JOIN Projects p ON p.id = t.projectId
WHERE t.id = :taskId;
