
#[cfg(test)]
mod tests {
    use e05_basic_testing::{math, shout, get_index};

    #[test]
    fn test_can_sum_positive_numbers() {
        let result: f64 = math::add(12.20, 21.34);

        assert_eq!(result, 33.54);
    }

    #[test]
    fn test_can_sum_negative_numbers() {
        let result: f64 = math::add(-12.20, 21.34);

        assert_eq!(result, 9.14);
    }

    #[test]
    fn test_can_convert_string_to_uppercase() {
        let word: String = String::from("Hello World!"); 
        let result: String = shout(&word);

        assert_eq!(result, word.to_uppercase());
    }

    #[test]
    fn test_can_get_item_in_an_array_if_index_is_within_range() {
        let v: Vec<i32> = vec![1, 3, 3, 4];
        assert_eq!(3, get_index(&v, 1))
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_should_panic_if_index_is_out_of_range() {
        let v: Vec<i32> = vec![1, 3, 3, 4];
        get_index(&v, 7);
    }
}