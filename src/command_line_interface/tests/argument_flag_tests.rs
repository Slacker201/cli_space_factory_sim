#[cfg(test)]
mod tests {
    use crate::command_line_interface::argument_flag::ArgumentFlag;

    #[test]
    fn display() {
        // Arrange
        let boolean_flag = ArgumentFlag::BooleanTrue;
        let value_flag = ArgumentFlag::Value("Foo".to_string());
        let value_flag_empty = ArgumentFlag::Value("".to_string());

        // Act

        // Assert
        assert_eq!("True", format!("{}", boolean_flag));
        assert_eq!("Foo", format!("{}", value_flag));
        assert_eq!("", format!("{}", value_flag_empty));
    }
}
