#[derive(Debug, PartialEq, Eq, strum_macros::EnumString, strum_macros::Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Command {
    Quit,
}

impl Command {
    pub fn parse(input: &str) -> Result<Self, strum::ParseError> {
        if let Some(input) = input.strip_prefix('/') {
            input.parse()
        } else {
            Err(strum::ParseError::VariantNotFound)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_command_from_str() {
        assert_eq!("quit".parse(), Ok(Command::Quit));
    }

    #[test]
    fn test_command_to_string() {
        assert_eq!(Command::Quit.to_string(), "quit");
    }

    #[test]
    fn test_parse_str_with_prefix() {
        assert_eq!(Command::parse("/quit"), Ok(Command::Quit));
    }
}
