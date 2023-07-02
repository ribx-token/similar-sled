use rocket::{get, State};
use rocket_contrib::json::Json;
use crate::models::{SimilarOutput, Database};
use crate::db::similars_all;

#[get("/verse_similars/<chapter>")]
pub fn get_verse_similar_by_chapter_route(
    dbs: State<Database>,
    chapter: u32,
) -> Json<Vec<SimilarOutput>>{
    let result = get_chapter_similars(&dbs, chapter);
    Json(result)
}

fn get_chapter_similars(dbs: &Database, chapter: u32) -> Vec<SimilarOutput> {
     let chapter_key = chapter.to_string();
    let similar_keys = get_similar_keys(dbs, &chapter_key);
    let mut similar_objects: Vec<SimilarOutput> = Vec::new();

    for similar_key in similar_keys {
        let similar = similars_all::get_similars_db_by_key(dbs, &similar_key);
        similar_objects.extend(similar);
    }
    
    similar_objects
}

fn get_similar_keys(dbs: &Database, key: &str) -> Vec<String> {
    let serialized_keys = dbs
        .verse_similar_db
        .get(key.as_bytes())
        .unwrap_or(None)
        .unwrap_or_default();

    bincode::deserialize(&serialized_keys)
        .unwrap_or_default()
}