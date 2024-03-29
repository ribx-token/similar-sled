use bincode;
use crate::models::chapter::Chapter;
use crate::models::database::Database;

pub fn get(dbs: &Database, chapter_no: u8) -> sled::Result<Option<String>> {
    let chapter_db = &dbs.chapter_db;
    let key = chapter_no.to_be_bytes().to_vec();
    match chapter_db.get(&key)? {
        Some(value) => {
            let chapter: Chapter = bincode::deserialize(&value).expect("Failed to deserialize");
            Ok(Some(chapter.sourate))
        },
        None => Ok(None),
    }
}