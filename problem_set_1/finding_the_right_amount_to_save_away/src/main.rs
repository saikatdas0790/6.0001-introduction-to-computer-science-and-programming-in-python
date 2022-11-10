use std::io;

use finding_the_right_amount_to_save_away::calculate_best_savings_rate;

fn main() {
    println!("Enter the starting salary: ");
    let mut starting_salary = String::new();
    io::stdin()
        .read_line(&mut starting_salary)
        .expect("Failed to read starting salary");
    let starting_salary: f64 = starting_salary
        .trim()
        .parse()
        .expect("Starting salary needs to be a number!");

    let value_returned = calculate_best_savings_rate(starting_salary);
    match value_returned {
        Some((best_savings_rate, steps_in_search)) => {
            println!("Best savings rate: {}", best_savings_rate);
            println!("Number of steps in bisection search: {}", steps_in_search)
        }
        None => println!("It is not possible to pay the down payment in three years."),
    }
}
