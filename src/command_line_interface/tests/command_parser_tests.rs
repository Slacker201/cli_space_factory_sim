#[cfg(test)]
mod tests {
    use crate::command_line_interface::{
        argument_flag::ArgumentFlag, command_dispatcher::parse_multiparam,
    };

    #[test]
    fn parses_flags_correctly() {
        // Arrange
        let c1 = "cmd --arg arg_val --bflag";
        //Act
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let parsed_args = parse_multiparam(&parts[1..]);
        // Assert
        assert!(parsed_args.contains_key("arg"));
        assert!(parsed_args.contains_key("bflag"));
    }
    #[test]
    fn parse_collects_single_arg() {
        // Arrange
        let c1 = "command --arg_name1 arg_val1";
        let parts: Vec<&str> = c1.split_whitespace().collect();

        // Act
        let arg_parts = &parts[1..];
        let parsed_args = parse_multiparam(arg_parts);

        // Assert
        assert_eq!(
            parsed_args["arg_name1"],
            vec![ArgumentFlag::Value("arg_val1".to_string())]
        );
    }

    #[test]
    fn parse_collects_multiple_arg_names_under_one_flag() {
        // Arrange
        let c1 = "command --arg_name1 arg_val1 --arg_name1 arg_val2";
        let parts: Vec<&str> = c1.split_whitespace().collect();

        // Act
        let arg_parts = &parts[1..];
        let parsed_args = parse_multiparam(arg_parts);

        // Assert
        assert_eq!(
            parsed_args["arg_name1"],
            vec![
                ArgumentFlag::Value("arg_val1".to_string()),
                ArgumentFlag::Value("arg_val2".to_string())
            ]
        );
    }

    #[test]
    fn parse_collects_bool_flags() {
        // Arrange
        let c1 = "command --arg_name1";
        let parts: Vec<&str> = c1.split_whitespace().collect();

        // Act

        let arg_parts = &parts[1..];
        let parsed_args = parse_multiparam(arg_parts);

        // Assert
        assert_eq!(parsed_args["arg_name1"], vec![ArgumentFlag::BooleanTrue]);
    }

    #[test]
    fn parse_collects_multiple_bool_flags() {
        // Arrange
        let c1 = "cmd --flag1 --flag2";

        // Act
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let args = parse_multiparam(&parts[1..]);
        // Assert
        assert_eq!(args["flag1"], vec![ArgumentFlag::BooleanTrue]);
        assert_eq!(args["flag2"], vec![ArgumentFlag::BooleanTrue]);
    }
    #[test]
    fn parse_collects_mixed_flags_two_bools() {
        // Arrange
        let c1 = "cmd --flag1 --flag2 bar --flag3";

        // Act
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let args = parse_multiparam(&parts[1..]);
        // Assert
        assert_eq!(args["flag1"], vec![ArgumentFlag::BooleanTrue]);
        assert_eq!(args["flag2"], vec![ArgumentFlag::Value("bar".to_string())]);
        assert_eq!(args["flag3"], vec![ArgumentFlag::BooleanTrue]);
    }
    #[test]
    fn empty_command() {
        // Arrange
        let c1 = "";

        // Act
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let args = parse_multiparam(&parts);
        // Assert
        assert!(args.is_empty())
    }
    #[test]
    fn duplicate_value_flags_with_unique_values() {
        // Arrange
        let c1 = "cm --flag foo --flag bar";

        // Act
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let args = parse_multiparam(&parts);
        // Assert
        assert_eq!(
            args["flag"],
            vec![
                ArgumentFlag::Value("foo".to_string()),
                ArgumentFlag::Value("bar".to_string())
            ]
        )
    }
    #[test]
    fn duplicate_value_flags_with_duplicate_values() {
        // Arrange
        let c1 = "cm --flag foo --flag foo";

        // Act
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let args = parse_multiparam(&parts);
        // Assert
        assert_eq!(
            args["flag"],
            vec![
                ArgumentFlag::Value("foo".to_string()),
                ArgumentFlag::Value("foo".to_string())
            ]
        )
    }
    #[test]
    fn second_value_without_flag_is_ignored() {
        // Arrange
        let c1 = "cmd --flag foo bar";

        // Act
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let args = parse_multiparam(&parts);
        // Assert
        assert_eq!(args["flag"], vec![ArgumentFlag::Value("foo".to_string())])
    }
    #[test]
    fn values_without_flags_are_ignored() {
        // Arrange
        let c1 = "cmd boo --flag far";

        // Act
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let args = parse_multiparam(&parts);
        // Assert
        assert_eq!(args["flag"], vec![ArgumentFlag::Value("far".to_string())])
    }
    #[test]
    fn single_dash_results_in_val_ignored_if_used_as_flag() {
        // Arrange
        let c1 = "cmd -boo bar --flag far";

        // Act
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let args = parse_multiparam(&parts);
        // Assert
        assert_eq!(args["flag"], vec![ArgumentFlag::Value("far".to_string())])
    }
    #[test]
    fn single_dash_value_used_as_value() {
        // Arrange
        let c1 = "cmd --flag -far";

        // Act
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let args = parse_multiparam(&parts);
        // Assert
        assert_eq!(args["flag"], vec![ArgumentFlag::Value("-far".to_string())])
    }
}