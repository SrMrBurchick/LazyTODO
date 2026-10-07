SELECT
    st.id,
    st.title,
    st.description,
    st.state,
    st.parentTaskId,
    t.title AS parentTaskTitle,
    t.projectId,
    p.title AS projectTitle
FROM v_GetSubTasks st
JOIN Tasks t on t.id = st.parentTaskId
LEFT JOIN Projects p on p.id = t.projectId
WHERE st.id = :subTaskId;
