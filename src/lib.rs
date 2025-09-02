pub mod calc_with_1 {

    ///    # This function sums and then adds plus 1
    ///    ## Example
    ///    ```rust
    ///    use calc_near_x_ijm::calc_with_1::sum_plus_one;
    ///
    ///    assert_eq!(42, sum_plus_one(41,0));
    ///    assert_eq!(4, sum_plus_one(1,2));
    ///    assert_eq!(1, sum_plus_one(0,0));
    pub fn sum_plus_one(x: u8, y: u8) -> u8 {
        return x + y + 1;
    }

    ///    # This function subtracts and then decreases minus 1
    ///     - If x less than or equals to y, returns 0.
    ///
    ///    ## Example
    ///    ```rust
    ///    use calc_near_x_ijm::calc_with_1::sub_minus_one;
    ///
    ///    assert_eq!(40, sub_minus_one(41,0));
    ///    assert_eq!(0, sub_minus_one(6,6));
    ///    assert_eq!(0, sub_minus_one(0,50));
    pub fn sub_minus_one(x: u8, y: u8) -> u8 {
        if x <= y {
            return 0;
        }

        return x - y - 1;
    }
}

#[cfg(test)]
mod test {

    use super::calc_with_1;

    #[test]
    fn test_sum() {
        let result = calc_with_1::sum_plus_one(5, 6);

        let msg = format!("The value is: {}", result);
        println!("{}", msg);

        assert_eq!(12, result);
    }

    #[test]
    fn test_sub_x_less_than_y() {
        let result = calc_with_1::sub_minus_one(5, 6);

        let msg = format!("The value is: {}", result);
        println!("{}", msg);

        assert_eq!(0, result);
    }

    #[test]
    fn test_sub_x_equals_y() {
        let result = calc_with_1::sub_minus_one(6, 6);

        let msg = format!("The value is: {}", result);
        println!("{}", msg);

        assert_eq!(0, result);
    }

    #[test]
    fn test_sub_x_greater_than_y() {
        let result = calc_with_1::sub_minus_one(6, 5);

        let msg = format!("The value is: {}", result);
        println!("{}", msg);

        assert_eq!(0, result);
    }

    #[test]
    fn test_sub_x_much_greater_than_y() {
        let result = calc_with_1::sub_minus_one(10, 5);

        let msg = format!("The value is: {}", result);
        println!("{}", msg);

        assert_eq!(4, result);
    }
}
