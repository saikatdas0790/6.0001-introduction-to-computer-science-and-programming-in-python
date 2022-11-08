pub fn raise_to_the_power(x: f64, y: f64) -> f64 {
    x.powf(y)
}

pub fn get_log_base_2(x: f64) -> f64 {
    x.log2()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_raise_to_the_power() {
        assert_eq!(raise_to_the_power(2.0, 3.0), 8.0);
        assert_eq!(raise_to_the_power(3.0, 2.0), 9.0);
        assert_eq!(raise_to_the_power(4.0, 1.0), 4.0);
        assert_eq!(raise_to_the_power(5.0, 0.0), 1.0);
    }

    #[test]
    fn test_get_log_base_2() {
        assert_eq!(get_log_base_2(2.0), 1.0);
        assert_eq!(get_log_base_2(4.0), 2.0);
        assert_eq!(get_log_base_2(8.0), 3.0);
        assert_eq!(get_log_base_2(16.0), 4.0);
    }
}
