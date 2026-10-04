SELECT
    COUNT(*) AS total,
    SUM(
        CASE
            WHEN EXISTS (
                SELECT 1
                FROM Tasks t
                WHERE t.projectId = p.id
            )
            AND NOT EXISTS (
                SELECT 1
                FROM Tasks t
                WHERE t.projectId = p.id
                  AND t.state != 2
            )
            THEN 1
            ELSE 0
        END
    ) AS completed
FROM Projects p;
