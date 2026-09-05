CREATE VIEW IF NOT EXISTS v_GetNonProjectTasks AS
    SELECT * FROM Tasks AS t
    WHERE t.projectId IS NULL;

CREATE VIEW IF NOT EXISTS v_GetProjectsTasks AS
    SELECT * FROM Tasks AS t
    WHERE t.projectId IS NOT NULL;

CREATE VIEW IF NOT EXISTS v_GetSubTasks AS
    SELECT * FROM SubTasks;

CREATE VIEW IF NOT EXISTS v_GetProjects AS
    SELECT * FROM Projects;
