use std::path::PathBuf;
use crate::models::Database;
use crate::db::chapter::chapters_init;
use crate::db::similar::similars_init;

pub fn init(absolute_data_folder_path: &PathBuf) -> Database {
    // Construct the absolute paths to the database files
    let chapter_db_path = absolute_data_folder_path.join("chapter_db");
    let verse_db_path = absolute_data_folder_path.join("verse_db");
    let similar_db_path = absolute_data_folder_path.join("similar_db");
    let verse_similar_db_path = absolute_data_folder_path.join("verse_similar_db");

    // Open the Sled databases using the adjusted file paths
    let chapter_db = sled::open(chapter_db_path).expect("Failed to open chapter database");
    let verse_db = sled::open(verse_db_path).expect("Failed to open verse database");
    let similar_db = sled::open(similar_db_path).expect("Failed to open similar database");
    let verse_similar_db = sled::open(verse_similar_db_path).expect("Failed to open similar database");

    let database = Database {
        chapter_db,
        verse_db,
        similar_db,
        verse_similar_db,
    };

    chapters_init::init(&database);
    similars_init::init(&database);
    database
}