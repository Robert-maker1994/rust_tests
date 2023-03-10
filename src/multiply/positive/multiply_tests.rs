
#[cfg(test)]
mod tests {
    // you need to import what functions you are using
    use crate::multiply::multiply;

    #[test]
    fn should_multiply_two_numbers() {
        let result = multiply(2,3);
        assert_eq!(result, 6);
    }
}