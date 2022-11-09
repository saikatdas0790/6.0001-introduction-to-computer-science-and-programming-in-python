use std::io;

use saving_with_a_raise::calculate_number_of_months;

fn main() {
    println!("Enter your annual salary: ");
    let mut annual_salary = String::new();
    io::stdin()
        .read_line(&mut annual_salary)
        .expect("Failed to read annual salary");
    let annual_salary: f64 = annual_salary
        .trim()
        .parse()
        .expect("Annual salary needs to be a number!");

    println!("Enter the percent of your salary to save, as a decimal: ");
    let mut portion_saved = String::new();
    io::stdin()
        .read_line(&mut portion_saved)
        .expect("Failed to read portion saved");
    let portion_saved: f64 = portion_saved
        .trim()
        .parse()
        .expect("Portion Saved needs to be a floating point number!");

    println!("Enter the cost of your dream home: ");
    let mut total_cost = String::new();
    io::stdin()
        .read_line(&mut total_cost)
        .expect("Failed to read portion saved");
    let total_cost: f64 = total_cost
        .trim()
        .parse()
        .expect("Portion Saved needs to be a floating point number!");

    println!("Enter the semi­annual raise, as a decimal: ");
    let mut semi_annual_raise = String::new();
    io::stdin()
        .read_line(&mut semi_annual_raise)
        .expect("Failed to read portion saved");
    let semi_annual_raise: f64 = semi_annual_raise
        .trim()
        .parse()
        .expect("Semi Annual Raise needs to be a floating point number!");

    let number_of_months =
        calculate_number_of_months(annual_salary, portion_saved, total_cost, semi_annual_raise);
    println!("Number of months: {}", number_of_months);
}
