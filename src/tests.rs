#[cfg(test)]
mod tests {
    use crate::core::database_manager::{Database};

    #[test]
    fn db_check() {
        let mut database : Database = Database::default();
        database.initialize("LazyTODO.sql");
        database.create_database();
    }
}
