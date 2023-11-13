use crate::models::{Database, Similar, VerseOutput, ExerciseOutput, Statement};
use crate::domain::similar::sourate_from_verse::sourate_name_from_verse;
use crate::utils::is_in_range::is_in_range;

pub fn create(dbs: &Database, similar: &Similar, ranges: &Option<Vec<(u8, u8)>>) -> ExerciseOutput {
    let similar_db = &dbs.similar_db;

    let verses_from_similar = similar.verses.iter()
        .filter(|verse| is_in_range(&verse.chapter_no, ranges))
        .map(|verse| create_statement(dbs, verse, &similar.kalima, similar.opposites.as_ref().map_or(false, |o| !o.is_empty())));

    let verses_from_opposites = similar.opposites.iter()
        .flat_map(|opposites| opposites.iter())
        .filter_map(|kalima| similar_db.get(kalima).ok().flatten())
        .filter_map(|data| bincode::deserialize::<Similar>(&data).ok())
        .flat_map(|similar| similar.verses.clone())  // clone verses here
        .filter(|verse| is_in_range(&verse.chapter_no, ranges))
        .map(|verse| create_statement(dbs, &verse, &similar.kalima, !similar.opposites.as_ref().unwrap_or(&Vec::new()).is_empty()));
    
    let all_verses = verses_from_similar.chain(verses_from_opposites).collect();

    ExerciseOutput {
        kalima: similar.kalima.clone(),
        verses: all_verses,
    }
}

fn create_statement(dbs: &Database, verse: &VerseOutput, kalima: &String, has_opposites: bool) -> Statement {
    let mut modified_verse = verse.clone();
    modified_verse.sourate = Some(sourate_name_from_verse(dbs, verse));
    Statement {
        verse: modified_verse,
        kalima: kalima.clone(),
        has_opposites,
    }
}