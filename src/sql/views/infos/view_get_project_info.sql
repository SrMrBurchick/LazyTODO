SELECT
    p.id,
    p.title,
    p.description,
    info.total,
    info.completed,
    CASE
        WHEN info.total = 0 THEN 0
        ELSE ROUND(100.0 * info.completed / info.total)
    END AS progress
FROM Projects p
CROSS JOIN (
    SELECT
        COUNT(*) AS total,
        SUM(
            CASE
                WHEN EXISTS (
                    SELECT 1
                    FROM SubTasks st
                    WHERE st.parentTaskId = t.id
                )
                THEN
                    CASE
                        WHEN NOT EXISTS (
                            SELECT 1
                            FROM SubTasks st
                            WHERE st.parentTaskId = t.id
                              AND st.state != 2
                        )
                        THEN 1
                        ELSE 0
                    END

                WHEN t.state = 2 THEN 1
                ELSE 0
            END
        ) AS completed
    FROM v_GetProjectsTasks t
    WHERE t.projectId = :projectId
) info
WHERE p.id = :projectId;
