CREATE TABLE IF NOT EXISTS Projects (
    id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT NULL,
    PRIMARY KEY(id ASC)
);

-- State
-- 0 = TODO
-- 1 = InProgress
-- 2 = Completed
CREATE TABLE IF NOT EXISTS Tasks (
    id INTEGER NOT NULL,
    title TEXT NOT NULL,
    projectId INTEGER NULL,
    state INTEGER NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY(id ASC)
);

CREATE TABLE IF NOT EXISTS SubTasks (
    id INTEGER NOT NULL,
    title TEXT NOT NULL,
    parentTaskId INTEGER NOT NULL,
    state INTEGER NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY(id ASC)
);
