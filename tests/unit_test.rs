#[cfg(test)]
mod tests {
    use rusty_encoder::base64encoder::{encode, get_lookup_value, is_padding_necessary};

    #[test]
    #[should_panic]
    fn invalid_lookup_values_result_in_panic() {
        get_lookup_value(68);
    }

    #[test]
    fn valid_lookup_values_match() {
        let expected_result = 'A';
        let actual_result = get_lookup_value(0);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn encoding_empty_string_returns_empty() {
        let expected_result = "";
        let empty_bytes: Vec<u8> = Vec::new();
        let actual_result = encode(&empty_bytes);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn encoding_values_without_padding() {
        let test_string = "helloworl";
        let test_string_bytes = test_string.as_bytes().to_vec();

        assert_eq!(is_padding_necessary(&test_string_bytes), false);

        let expected_result = "aGVsbG93b3Js";
        let actual_result = encode(&test_string_bytes);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn encoding_values_with_padding() {
        let test_string = "M";
        let test_string_bytes = test_string.as_bytes().to_vec();

        assert_eq!(is_padding_necessary(&test_string_bytes), true);

        let expected_result = "TQ==";
        let actual_result = encode(&test_string_bytes);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn encoding_values_with_padding_longer() {
        let test_string = "helloworld";
        let test_string_bytes = test_string.as_bytes().to_vec();

        assert_eq!(is_padding_necessary(&test_string_bytes), true);

        let expected_result = "aGVsbG93b3JsZA==";
        let actual_result = encode(&test_string_bytes);

        assert_eq!(expected_result, actual_result);
    }
}
