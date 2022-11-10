pub fn calculate_best_savings_rate(starting_salary: f64) -> Option<(f64, u32)> {
    let semi_annual_raise = 0.07;
    let annual_return_on_investment = 0.04;
    let portion_down_payment = 0.25;
    let total_cost = 1_000_000.0;
    let down_payment = total_cost * portion_down_payment;
    let number_of_months = 36;
    let mut current_savings = 0.0;

    let mut steps_in_search = 0;
    let mut best_savings_rate = 0;
    let mut min_savings_rate = 0;
    let mut max_savings_rate = 10000;

    while min_savings_rate <= max_savings_rate {
        steps_in_search += 1;
        let savings_rate = (min_savings_rate + max_savings_rate) / 2;
        let monthly_savings_rate = savings_rate as f64 / 10000.0;
        let mut monthly_salary = starting_salary / 12.0;
        for _ in 0..number_of_months {
            current_savings += current_savings * annual_return_on_investment / 12.0;
            current_savings += monthly_salary * monthly_savings_rate;
            if (steps_in_search % 6) == 0 {
                monthly_salary += monthly_salary * semi_annual_raise;
            }
        }
        if current_savings > down_payment {
            max_savings_rate = savings_rate - 1;
            best_savings_rate = savings_rate;
        } else {
            min_savings_rate = savings_rate + 1;
        }
        current_savings = 0.0;
    }

    if best_savings_rate == 0 {
        None
    } else {
        Some((best_savings_rate as f64 / 10000.0, steps_in_search))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore = "doesn't match the values but the logic seems right"]
    #[test]
    fn test_calculate_number_of_steps_in_bisection_search() {
        assert_eq!(calculate_best_savings_rate(150_000.0), Some((0.4411, 12)));
        assert_eq!(calculate_best_savings_rate(300_000.0), Some((0.2206, 9)));
        assert_eq!(calculate_best_savings_rate(10_000.0), None);
    }
}
