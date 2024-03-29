use rand::seq::SliceRandom;
use crate::domain::exercise::extract_and_shuffle_options::extract_and_shuffle_options;
use crate::domain::exercise::select_random_verse_index::select_random_verse_index;
use crate::domain::similar::sourate_from_verse::sourate_name_from_verse;
use crate::models::{
    exercise_type::ExerciseType, exercise_output::ExerciseOutput, database::Database,
    alternative::Alternative, exercise::Exercise, verse_output::VerseOutput, ungrouped_text::UngroupedText
};

pub fn generate_one_exercise(dbs: &Database, exercise: &mut ExerciseOutput, exercise_type: ExerciseType) -> Option<Exercise> {
    let valid_verse_index = select_random_verse_index(&exercise.verses);

    if let Some(ref mut valid_verse) = exercise.verses.get_mut(valid_verse_index) {
        valid_verse.verse.sourate = Some(sourate_name_from_verse(dbs, &valid_verse.verse));
    }
    let exclude_verse = Some(exercise.verses[valid_verse_index].verse.clone());
    let extracted_values = extract_and_shuffle_options(&mut exercise.verses, exercise_type, &exclude_verse);

    // Extract and keep the correct answer separate
    let correct_alternative = Alternative { verse: Some(exercise.verses[valid_verse_index].verse.clone()) };

    // Prepare incorrect alternatives and shuffle them
    let mut incorrect_alternatives: Vec<Alternative> = extracted_values.into_iter().map(|value| {
    
        match exercise_type {
            ExerciseType::FindDiscriminant => {
                Alternative {
                    verse: Some(VerseOutput {
                        chapter_no: value.1.chapter_no,
                        verse_no: value.1.verse_no,
                        sourate: value.1.sourate,
                        ungrouped_text: Some(UngroupedText {
                            discriminant: Some(value.0),
                            pre: None,
                            post: None,
                        }),
                    }),
                }
            },
            ExerciseType::FindSourate => {
                Alternative {
                    verse: Some(VerseOutput {
                        chapter_no: value.1.chapter_no,
                        verse_no: value.1.verse_no,
                        sourate: Some(value.0),
                        ungrouped_text: None,
                    }),
                }
            },
            _ => unimplemented!(),
        }
    }).filter(|alt| alt.verse != Some(exercise.verses[valid_verse_index].verse.clone()))
      .collect();

    incorrect_alternatives.truncate(4);
    incorrect_alternatives.push(correct_alternative);
    incorrect_alternatives.shuffle(&mut rand::thread_rng());

    // Use the combined and truncated list as the final alternatives
    let alternatives = incorrect_alternatives;

    // Return None if there are not enough alternatives
    if alternatives.len() <= 1 {
        None
    } else {
        let mut generated_exercise = Exercise {
            statement: exercise.verses[valid_verse_index].clone(),
            alternatives,
            exercise_type: exercise_type.clone(),
        };

        exercise_type.hide_fields(&mut generated_exercise);

        Some(generated_exercise)
    }
}
