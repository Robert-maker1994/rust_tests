
#[cfg(test)]
mod tests {
    use crate::subtract::sub;

    #[test]
    fn should_sub_two_numbers() {
        let result = sub(2,3);
        assert_eq!(result, -1);
    }
}