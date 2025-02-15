use std::collections::HashMap;
use std::io;

// Define a struct to represent a workout
struct Workout {
    name: String,
    exercises: Vec<Exercise>,
}

// Define a struct to represent an exercise
struct Exercise {
    name: String,
    sets: u32,
    reps: u32,
    weight: f64,
}

// Define a struct to represent a user's fitness goals
struct FitnessGoals {
    goal: String,
    target_value: f64,
}

// Define a struct to represent a user's workout history
struct WorkoutHistory {
    workouts: Vec<Workout>,
}

// Implement methods for the Workout struct
impl Workout {
    fn new(name: String) -> Self {
        Workout {
            name,
            exercises: Vec::new(),
        }
    }

    fn add_exercise(&mut self, exercise: Exercise) {
        self.exercises.push(exercise);
    }
}

// Implement methods for the Exercise struct
impl Exercise {
    fn new(name: String, sets: u32, reps: u32, weight: f64) -> Self {
        Exercise {
            name,
            sets,
            reps,
            weight,
        }
    }
}

// Implement methods for the FitnessGoals struct
impl FitnessGoals {
    fn new(goal: String, target_value: f64) -> Self {
        FitnessGoals {
            goal,
            target_value,
        }
    }
}

// Implement methods for the WorkoutHistory struct
impl WorkoutHistory {
    fn new() -> Self {
        WorkoutHistory {
            workouts: Vec::new(),
        }
    }

    fn add_workout(&mut self, workout: Workout) {
        self.workouts.push(workout);
    }
}

fn main() {
    let mut user_workout_history = WorkoutHistory::new();
    let mut user_fitness_goals = FitnessGoals::new("Weight Loss".to_string(), 10.0);

    loop {
        println!("1. Record a workout");
        println!("2. View workout history");
        println!("3. Set fitness goals");
        println!("4. View fitness goals");
        println!("5. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => {
                let mut workout_name = String::new();
                println!("Enter workout name:");
                io::stdin().read_line(&mut workout_name).expect("Failed to read line");

                let mut workout = Workout::new(workout_name.trim().to_string());

                loop {
                    println!("1. Add exercise");
                    println!("2. Finish workout");

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");

                    match input.trim() {
                        "1" => {
                            let mut exercise_name = String::new();
                            println!("Enter exercise name:");
                            io::stdin().read_line(&mut exercise_name).expect("Failed to read line");

                            let mut sets = String::new();
                            println!("Enter number of sets:");
                            io::stdin().read_line(&mut sets).expect("Failed to read line");

                            let mut reps = String::new();
                            println!("Enter number of reps:");
                            io::stdin().read_line(&mut reps).expect("Failed to read line");

                            let mut weight = String::new();
                            println!("Enter weight:");
                            io::stdin().read_line(&mut weight).expect("Failed to read line");

                            let exercise = Exercise::new(
                                exercise_name.trim().to_string(),
                                sets.trim().parse().expect("Failed to parse"),
                                reps.trim().parse().expect("Failed to parse"),
                                weight.trim().parse().expect("Failed to parse"),
                            );

                            workout.add_exercise(exercise);
                        }
                        "2" => {
                            break;
                        }
                        _ => {
                            println!("Invalid input");
                        }
                    }
                }

                user_workout_history.add_workout(workout);
            }
            "2" => {
                for workout in &user_workout_history.workouts {
                    println!("Workout: {}", workout.name);
                    for exercise in &workout.exercises {
                        println!("Exercise: {}", exercise.name);
                        println!("Sets: {}", exercise.sets);
                        println!("Reps: {}", exercise.reps);
                        println!("Weight: {}", exercise.weight);
                    }
                }
            }
            "3" => {
                let mut goal = String::new();
                println!("Enter fitness goal:");
                io::stdin().read_line(&mut goal).expect("Failed to read line");

                let mut target_value = String::new();
                println!("Enter target value:");
                io::stdin().read_line(&mut target_value).expect("Failed to read line");

                user_fitness_goals = FitnessGoals::new(goal.trim().to_string(), target_value.trim().parse().expect("Failed to parse"));
