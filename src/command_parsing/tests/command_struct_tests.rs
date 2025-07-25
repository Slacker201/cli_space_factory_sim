#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::command_parsing::{command_struct::Command, command_token::CommandToken};

    #[test]
    fn default_values_and_getters() {
        let cmd = Command::new();
        assert_eq!(cmd.name(), "".to_string());
        assert!(cmd.args().is_empty());
    }
    #[test]
    fn name_setter() {
        let mut cmd = Command::new();
        cmd.set_name("bar".to_string());
        assert_eq!(cmd.name(), "bar")
    }
    #[test]
    fn arg_setter() {
        let mut cmd = Command::new();
        let mut hash_map = HashMap::new();
        hash_map.insert(
            "foo".to_string(),
            vec![
                CommandToken::BooleanTrue,
                CommandToken::Value("bar".to_string()),
            ],
        );
        cmd.set_args(hash_map);
        assert_eq!(
            cmd.args().get("foo"),
            Some(&vec![
                CommandToken::BooleanTrue,
                CommandToken::Value("bar".to_string())
            ])
        )
    }
}
