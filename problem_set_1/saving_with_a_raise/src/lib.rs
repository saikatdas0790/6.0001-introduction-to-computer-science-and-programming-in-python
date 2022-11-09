pub fn calculate_number_of_months(
    mut annual_salary: f64,
    portion_saved: f64,
    total_cost: f64,
    semi_annual_raise: f64,
) -> i32 {
    let mut current_savings = 0.0;
    let r = 0.04;
    let mut monthly_salary = annual_salary / 12.0;
    let portion_down_payment = 0.25;
    let down_payment = total_cost * portion_down_payment;
    let mut number_of_months = 0;
    while current_savings < down_payment {
        current_savings += current_savings * r / 12.0;
        current_savings += monthly_salary * portion_saved;
        number_of_months += 1;
        if number_of_months % 6 == 0 {
            annual_salary += annual_salary * semi_annual_raise;
            monthly_salary = annual_salary / 12.0;
        }
    }
    number_of_months
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_number_of_months() {
        assert_eq!(
            calculate_number_of_months(120_000.0, 0.05, 500_000.0, 0.03),
            142
        );
        assert_eq!(
            calculate_number_of_months(80_000.0, 0.1, 800_000.0, 0.03),
            159
        );
        assert_eq!(
            calculate_number_of_months(75_000.0, 0.05, 1_500_000.0, 0.05),
            261
        );
    }
}
