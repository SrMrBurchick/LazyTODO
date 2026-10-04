SELECT
    t.id,
    t.title,
    t.description,
    t.projectId,
    p.title AS projectTitle
    info.total,
    info.completed,
    CASE
        WHEN info.total = 0 THEN
            CASE
                WHEN t.state = 2 THEN 100
                ELSE 0
            END
        ELSE ROUND(100.0 * info.completed / info.total)
    END AS progress
FROM Tasks t
CROSS JOIN (
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
) info
LEFT JOIN Projects p ON p.id = t.projectId
WHERE t.id = :taskId;
