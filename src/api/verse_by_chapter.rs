use rocket::{get, State};
use rocket::serde::{json::Json};

use crate::models::database::Database;
use crate::domain::chapter::chapter;
use crate::domain::verse::verses_by_chapter::verses_by_chapter;

#[get("/verse/<chapter_no>")]
pub fn get_verse(chapter_no: u8, dbs: &State<Database>) -> Json<serde_json::Value> {
    let chapter = chapter::get(&dbs, chapter_no).unwrap();
    let verses = verses_by_chapter(&dbs, chapter_no).unwrap();

    // Create a JSON value using serde_json
    let json_value = serde_json::json!({
        "chapter": chapter,
        "verses": verses,
    });

    // Wrap the JSON value in a `Json` struct
    Json(json_value)
}