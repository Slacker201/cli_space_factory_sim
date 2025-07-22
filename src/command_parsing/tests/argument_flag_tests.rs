#[cfg(test)]
mod tests {
    use crate::command_parsing::command_token::CommandToken;

    #[test]
    fn display() {
        // Arrange
        let boolean_flag = CommandToken::BooleanTrue;
        let value_flag = CommandToken::Value("Foo".to_string());
        let value_flag_empty = CommandToken::Value("".to_string());

        // Act

        // Assert
        assert_eq!("True", format!("{}", boolean_flag));
        assert_eq!("Foo", format!("{}", value_flag));
        assert_eq!("", format!("{}", value_flag_empty));
    }
}
