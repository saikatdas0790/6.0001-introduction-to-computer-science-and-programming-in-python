pub fn calculate_number_of_months(annual_salary: f64, portion_saved: f64, total_cost: f64) -> i32 {
    let portion_down_payment = 0.25;
    let r = 0.04;
    let mut current_savings = 0.0;
    let mut number_of_months = 0;
    let monthly_salary = annual_salary / 12.0;
    while current_savings < total_cost * portion_down_payment {
        current_savings += current_savings * r / 12.0;
        current_savings += monthly_salary * portion_saved;
        number_of_months += 1;
    }
    number_of_months
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_number_of_months() {
        assert_eq!(
            calculate_number_of_months(120_000.0, 0.10, 1_000_000.0),
            183
        );
        assert_eq!(calculate_number_of_months(80_000.0, 0.15, 500_000.0), 105);
    }
}
