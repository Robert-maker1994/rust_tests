
#[cfg(test)]
mod tests {
    use crate::add::add;

    #[test]
    fn add_two_numbers() {
        let result = add(2,3);
        assert_eq!(result, 5);
    }
}