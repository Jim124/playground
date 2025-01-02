/// Function:subtract_10
///
/// # Argument(num:u32)
///
/// # Example
///
/// ```
/// let x = 5;
/// let y = subtract_10(x);
///
/// ```

pub fn subtract_10(num: u32) -> u32 {
    num - 10
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]

    fn subtracts_10_test() {
        let x: u32 = 100;
        let y: u32 = subtract_10(x);
        println!("x and y are from test {},{}", x, y);
        assert_eq!(y, 90);
    }
}
