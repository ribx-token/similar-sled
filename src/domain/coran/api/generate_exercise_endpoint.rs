use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::domain::coran::models::VerseUngrouped;
use crate::models::Database;
use crate::domain::coran::db::exercise::find_discriminant;

#[get("/exercise/<kalima>")]
pub fn generate_exercise_endpoint(kalima: String, dbs: State<Database>) -> Option<Json<(VerseUngrouped, Vec<String>)>> {
   find_discriminant::generate(&dbs, kalima).map(Json)
}
