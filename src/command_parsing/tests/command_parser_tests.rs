#[cfg(test)]
mod tests {

    use crate::{
        command_parsing::{
            command_dispatcher::{parse, CommandParseError}, command_token::CommandToken,
        },
        error,
    };

    #[test]
    fn parses_flags_correctly() {
        let c1 = "cmd --arg arg_val --bflag";
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let cmd = match parse(parts) {
            Ok(cmd) => cmd,
            Err(e) => {
                error!("{}", e);
                panic!("Failed to parse command")
            }
        };
        let parsed_args = cmd.args();
        assert!(parsed_args.contains_key("arg"));
        assert!(parsed_args.contains_key("bflag"));
    }

    #[test]
    fn parse_collects_single_arg() {
        let c1 = "command --arg_name1 arg_val1";
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let cmd = match parse(parts) {
            Ok(cmd) => cmd,
            Err(e) => {
                error!("{}", e);
                panic!("Failed to parse command")
            }
        };
        let parsed_args = cmd.args();
        assert_eq!(
            parsed_args["arg_name1"],
            vec![CommandToken::Value("arg_val1".to_string())]
        );
    }

    #[test]
    fn parse_collects_multiple_arg_names_under_one_flag() {
        let c1 = "command --arg_name1 arg_val1 --arg_name1 arg_val2";
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let cmd = match parse(parts) {
            Ok(cmd) => cmd,
            Err(e) => {
                error!("{}", e);
                panic!("Failed to parse command")
            }
        };
        let parsed_args = cmd.args();
        assert_eq!(
            parsed_args["arg_name1"],
            vec![
                CommandToken::Value("arg_val1".to_string()),
                CommandToken::Value("arg_val2".to_string())
            ]
        );
    }

    #[test]
    fn parse_collects_bool_flags() {
        let c1 = "command --arg_name1";
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let cmd = match parse(parts) {
            Ok(cmd) => cmd,
            Err(e) => {
                error!("{}", e);
                panic!("Failed to parse command")
            }
        };
        let parsed_args = cmd.args();
        assert_eq!(parsed_args["arg_name1"], vec![CommandToken::BooleanTrue]);
    }

    #[test]
    fn parse_collects_multiple_bool_flags() {
        let c1 = "cmd --flag1 --flag2";
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let cmd = match parse(parts) {
            Ok(cmd) => cmd,
            Err(e) => {
                error!("{}", e);
                panic!("Failed to parse command")
            }
        };
        let args = cmd.args();
        assert_eq!(args["flag1"], vec![CommandToken::BooleanTrue]);
        assert_eq!(args["flag2"], vec![CommandToken::BooleanTrue]);
    }

    #[test]
    fn parses_mixed_bool_and_value_flags() {
        let c1 = "cmd --flag1 --flag2 bar --flag3";
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let cmd = match parse(parts) {
            Ok(cmd) => cmd,
            Err(e) => {
                error!("{}", e);
                panic!("Failed to parse command")
            }
        };
        let args = cmd.args();
        assert_eq!(args["flag1"], vec![CommandToken::BooleanTrue]);
        assert_eq!(args["flag2"], vec![CommandToken::Value("bar".to_string())]);
        assert_eq!(args["flag3"], vec![CommandToken::BooleanTrue]);
    }

    #[test]
    fn empty_command() {
        let c1 = "";
        let parts: Vec<&str> = c1.split_whitespace().collect();
        match parse(parts) {
            Ok(_cmd) => {
                panic!("Command was made when it should not be");
            },
            Err(e) => {
                assert_eq!(e, CommandParseError::EmptyTokenArray);
                return;
            }
        };
    }

    #[test]
    fn duplicate_value_flags_with_unique_values() {
        let c1 = "cm --flag foo --flag bar";
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let cmd = match parse(parts) {
            Ok(cmd) => cmd,
            Err(e) => {
                error!("{}", e);
                panic!("Failed to parse command")
            }
        };
        let args = cmd.args();
        assert_eq!(
            args["flag"],
            vec![
                CommandToken::Value("foo".to_string()),
                CommandToken::Value("bar".to_string())
            ]
        );
    }

    #[test]
    fn duplicate_value_flags_with_duplicate_values() {
        let c1 = "cm --flag foo --flag foo";
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let cmd = match parse(parts) {
            Ok(cmd) => cmd,
            Err(e) => {
                error!("{}", e);
                panic!("Failed to parse command")
            }
        };
        let args = cmd.args();
        assert_eq!(
            args["flag"],
            vec![
                CommandToken::Value("foo".to_string()),
                CommandToken::Value("foo".to_string())
            ]
        );
    }

    #[test]
    fn multiple_values_after_flag_are_added_to_flag() {
        let c1 = "cmd --flag foo bar";
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let cmd = match parse(parts) {
            Ok(cmd) => cmd,
            Err(e) => {
                error!("{}", e);
                panic!("Failed to parse command")
            }
        };
        let args = cmd.args();
        assert_eq!(args["flag"], vec![CommandToken::Value("foo".to_string()), CommandToken::Value("bar".to_string())]);
    }

    #[test]
    fn values_declared_before_flag_are_ignored() {
        let c1 = "cmd boo --flag far";
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let cmd = match parse(parts) {
            Ok(cmd) => cmd,
            Err(e) => {
                error!("{}", e);
                panic!("Failed to parse command")
            }
        };
        let args = cmd.args();
        assert_eq!(args["flag"], vec![CommandToken::Value("far".to_string())]);
    }

    #[test]
    fn single_dash_results_in_val_ignored_if_used_as_flag() {
        let c1 = "cmd -boo bar --flag far";
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let cmd = match parse(parts) {
            Ok(cmd) => cmd,
            Err(e) => {
                error!("{}", e);
                panic!("Failed to parse command")
            }
        };
        let args = cmd.args();
        assert_eq!(args["flag"], vec![CommandToken::Value("far".to_string())]);
    }

    #[test]
    fn single_dash_value_used_as_value() {
        let c1 = "cmd --flag -far";
        let parts: Vec<&str> = c1.split_whitespace().collect();
        let cmd = match parse(parts) {
            Ok(cmd) => cmd,
            Err(e) => {
                error!("{}", e);
                panic!("Failed to parse command")
            }
        };
        let args = cmd.args();
        assert_eq!(args["flag"], vec![CommandToken::Value("-far".to_string())]);
    }
}
