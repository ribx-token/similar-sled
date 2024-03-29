use crate::domain::exercise::get_solution::get_solution;
use crate::domain::exercise::generate_one_exercise::generate_one_exercise;
use crate::models::{
    exercise_type::ExerciseType, database::Database, exercise::Exercise
};
use rand::seq::SliceRandom;
use crate::learning::models::analytic::Analytic;

pub fn get_exercises(
    dbs: &Database,
    analytics: &Vec<Analytic>,
    ranges: &Option<Vec<(u8, u8)>>
) -> Vec<Exercise> {
    let mut exercises = Vec::new();
    
    for exercise_type in [ExerciseType::FindSourate, ExerciseType::FindDiscriminant].iter() {
        let mut solutions = get_solution(dbs, ranges, analytics);
        let generated_exercises = solutions.iter_mut()
            .filter_map(|solution| generate_one_exercise(dbs, solution, exercise_type.clone()))
            .collect::<Vec<_>>();

        exercises.extend(generated_exercises);
    }

    // Shuffle the exercises
    let mut rng = rand::thread_rng();
    exercises.shuffle(&mut rng);

    exercises
}
