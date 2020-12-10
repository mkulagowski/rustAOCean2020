mod common;
mod days;

fn main() {
    for day in days::all_numbers() {
        if let Some(solver) = days::get_solver(day) {
            let input = common::get_day_input(day).expect(&format!(
                "Problem occured while getting input for day{:02}",
                day
            ));

            let (solution, time) = solver(&input);
            println!(
                "Solution for day{:02}: ({}, {}), took {:?}",
                day, solution.0, solution.1, time
            );
        }
    }
    println!("DONE");
}
