#[cfg(test)]
mod tests {
    use noted::db::*;
    use noted::note::NoteError;
    use noted::SortOrder;
    use rusqlite::Connection;
    use std::env;
    use std::fs;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::path::Path;
    use std::path::PathBuf;
    use std::{thread, time};
    use uuid::Uuid;

    struct TestDb {
        path: PathBuf,
    }

    impl TestDb {
        fn new(db_path: PathBuf) -> Self {
            TestDb { path: db_path }
        }

        fn conn(&self) -> Connection {
            Connection::open(&self.path).expect("Failed to open test database")
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            if self.path.exists() {
                fs::remove_file(&self.path).expect("Failed to remove test database");
            }
        }
    }

    fn read_file_to_vector(path: PathBuf) -> Result<Vec<String>, NoteError> {
        let file = File::open(path).map_err(NoteError::FileError)?;
        let buf = BufReader::new(file);

        buf.lines()
            .map(|l| l.map_err(NoteError::FileError))
            .collect()
    }

    fn init_test_db(db_name: &str) -> TestDb {
        let home_dir = env::var("HOME").expect("Could not get $HOME directory");
        let path = format!(".local/share/noted/{}", db_name);
        let db_path = PathBuf::from(home_dir).join(path);
        TestDb::new(db_path)
    }

    #[test]
    fn test_database_basics() {
        let db_name = "notes_basic_test.db";
        let test_db = init_test_db(db_name);
        let conn = test_db.conn();

        assert!(
            conn.is_autocommit(),
            "Connection should be in autocommit mode"
        );

        // Test creating a table
        let create_table_result = create_table(&conn);
        assert!(
            create_table_result.is_ok(),
            "Creating the note table should not fail"
        );

        let first_note = String::from("A sneeze (also known as sternutation) is a semi-autonomous, convulsive expulsion of air from the lungs through the nose and mouth, usually caused by foreign particles irritating the nasal mucosa.");
        let second_note= String::from("A sneeze expels air forcibly from the mouth and nose in an explosive, spasmodic involuntary action. This action allows for mucus to escape through the nasal cavity and saliva to escape from the oral cavity.");

        {
            // Test first insert
            let insertion_result = create_new_note(&conn, first_note.to_string());
            assert!(
                insertion_result.is_ok(),
                "Creating a new note should not fail"
            );
        }
        {
            // Test second insert
            let insertion_result = create_new_note(&conn, second_note.to_string());
            assert!(
                insertion_result.is_ok(),
                "Creating a second note should not fail"
            );
        }

        {
            // Test getting the first note
            let mut result = get_notes_with_qty_and_order(&conn, 1, SortOrder::Asc).unwrap();
            let note = result.pop();
            if let Some(n) = note {
                assert_eq!(first_note, n.get_content());
            }
        }

        // Test getting the second note
        let mut result = get_notes_with_qty_and_order(&conn, 1, SortOrder::Desc).unwrap();
        let mut note = result.pop();
        if let Some(n) = note {
            assert_eq!(first_note, n.get_content());

            // Test searching for a note
            let search_term = String::from("escape");
            result = search_notes_by_content(&conn, &search_term).unwrap();
            assert_eq!(1, result.len());
            note = result.pop();
            if let Some(n) = note {
                assert!(n.get_content().contains("spasmodic involuntary action"));
            }

            // Test deleting the note
            let id = String::from(n.get_id());
            let deletion_result = delete_note(&conn, &id).unwrap();
            assert_eq!(1, deletion_result);
        }

        // There should now be only one note in the db
        result = get_all_notes(&conn).unwrap();
        assert_eq!(1, result.len());

        // Test clearing the database
        let deleted_rows = delete_all_notes(&conn).unwrap();
        assert_eq!(1, deleted_rows);
        let all_notes_after_reset = get_all_notes(&conn).unwrap();
        assert_eq!(0, all_notes_after_reset.len());
    }

    #[test]
    fn test_large_noteset() {
        let db_name = "notes_large_dataset_test.db";
        let test_db = init_test_db(db_name);
        let conn = test_db.conn();
        let file_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/resources/large_noteset_data.txt");
        let dataset = read_file_to_vector(file_path).unwrap();

        create_table(&conn).unwrap();

        for quote in dataset {
            create_new_note(&conn, quote).unwrap();
        }

        let all_notes_from_db = get_all_notes(&conn).unwrap();

        assert!(
            all_notes_from_db.len() > 50,
            "There should be at least 50 notes in the db"
        );

        let search_term = String::from("what");
        let found_notes = search_notes_by_content(&conn, &search_term).unwrap();
        assert!(
            found_notes.len() >= 4,
            "There are surely at least 4 notes with the substring 'what'"
        );

        for note in found_notes {
            let uuid_result = Uuid::parse_str(note.get_id());
            assert!(uuid_result.is_ok(), "Invalid UUID: {}", note.get_id());
        }

        let invalid_id = String::from("6666666");
        let deletion_result = delete_note(&conn, &invalid_id).unwrap();
        assert_eq!(0, deletion_result, "Delete operation with invalid ID should not cause errors, but should not find anything to delete either");

        let deleted_rows = delete_all_notes(&conn).unwrap();
        assert!(
            deleted_rows > 50,
            "at least 50 notes should have been deleted",
        );

        let notes_in_empty_db = get_all_notes(&conn).unwrap();
        assert!(
            notes_in_empty_db.is_empty(),
            "There should be no notes in the db at this point"
        );
    }

    #[test]
    fn test_editing_notes() {
        let db_name = "test_edit_notes.db";
        let test_db = init_test_db(db_name);
        let conn = test_db.conn();
        let file_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/resources/noteset_to_edit.txt");
        let dataset = read_file_to_vector(file_path).unwrap();

        create_table(&conn).unwrap();

        for quote in dataset {
            create_new_note(&conn, quote).unwrap();
        }

        let notes = get_all_notes(&conn).unwrap();

        let first_note = notes.first().unwrap();
        let second_note = notes.get(1).unwrap();
        let third_note = notes.get(2).unwrap();

        assert_eq!(
            "Tonight, I intend to have some fun by coding.",
            first_note.get_content(),
            "Note should have correct content"
        );

        assert_eq!(
            "WTF did I code yesterday?",
            second_note.get_content(),
            "Note should have correct content"
        );

        assert_eq!(
            "Tomorrow I'll look at this spaghetti.",
            third_note.get_content(),
            "Note should have correct content"
        );

        let id = String::from(third_note.get_id());
        let content = third_note
            .get_content()
            .to_string()
            .replace("Tomorrow", "Now");

        // Wait for a second to give the timestamp chance to update
        let sec = time::Duration::from_millis(1200);
        thread::sleep(sec);

        let edited_rows = edit_note(&conn, &id, &content).unwrap();

        assert_eq!(1, edited_rows, "Editing should result in 1 updated row");

        let updated_notes = get_all_notes(&conn).unwrap();
        let updated_third_note = updated_notes.get(2).unwrap();

        assert_eq!(
            "Now I'll look at this spaghetti.",
            updated_third_note.get_content(),
            "The content of thins note should have been updated"
        );

        assert_eq!(
            third_note.get_id(),
            updated_third_note.get_id(),
            "The ID should not have been changed"
        );

        assert!(
            third_note.get_date() < updated_third_note.get_date(),
            "The date should have been updated"
        );
    }
}
