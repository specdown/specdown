#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ArgumentValue {
    Integer(i32),
    String(String),
    Token(String),
}

#[derive(Debug, Eq, PartialEq)]
pub struct IncorrectArgumentType {
    pub expected: String,
    pub got: String,
}

impl ArgumentValue {
    pub fn integer(&self) -> Result<i32, IncorrectArgumentType> {
        match self {
            Self::Integer(num) => Ok(*num),
            Self::String(_) => Self::incorrect_argument_type_error("integer", "string"),
            Self::Token(_) => Self::incorrect_argument_type_error("integer", "token"),
        }
    }

    pub fn string(&self) -> Result<String, IncorrectArgumentType> {
        match self {
            Self::String(s) => Ok(s.clone()),
            Self::Integer(_) => Self::incorrect_argument_type_error("string", "integer"),
            Self::Token(_) => Self::incorrect_argument_type_error("string", "token"),
        }
    }

    pub fn token(&self) -> Result<String, IncorrectArgumentType> {
        match self {
            Self::Token(t) => Ok(t.clone()),
            Self::Integer(_) => Self::incorrect_argument_type_error("token", "integer"),
            Self::String(_) => Self::incorrect_argument_type_error("token", "string"),
        }
    }

    fn incorrect_argument_type_error<T>(
        expected: &str,
        got: &str,
    ) -> Result<T, IncorrectArgumentType> {
        Err(IncorrectArgumentType {
            expected: expected.to_string(),
            got: got.to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::{ArgumentValue, IncorrectArgumentType};

    mod integer {
        use super::{ArgumentValue, IncorrectArgumentType};

        #[test]
        fn returns_integer_when_value_is_an_integer() {
            assert_eq!(Ok(4), ArgumentValue::Integer(4).integer());
        }

        #[test]
        fn returns_error_when_value_is_a_string() {
            assert_eq!(
                Err(IncorrectArgumentType {
                    expected: "integer".to_string(),
                    got: "string".to_string(),
                }),
                ArgumentValue::String("hello".to_string()).integer()
            );
        }

        #[test]
        fn returns_error_when_value_is_a_token() {
            assert_eq!(
                Err(IncorrectArgumentType {
                    expected: "integer".to_string(),
                    got: "token".to_string(),
                }),
                ArgumentValue::Token("hello".to_string()).integer()
            );
        }
    }

    mod string {
        use super::{ArgumentValue, IncorrectArgumentType};

        #[test]
        fn returns_string_when_value_is_a_string() {
            assert_eq!(
                Ok("value".to_string()),
                ArgumentValue::String("value".to_string()).string()
            );
        }

        #[test]
        fn returns_error_when_value_is_a_string() {
            assert_eq!(
                Err(IncorrectArgumentType {
                    expected: "string".to_string(),
                    got: "integer".to_string(),
                }),
                ArgumentValue::Integer(5).string()
            );
        }

        #[test]
        fn returns_error_when_value_is_a_token() {
            assert_eq!(
                Err(IncorrectArgumentType {
                    expected: "string".to_string(),
                    got: "token".to_string(),
                }),
                ArgumentValue::Token("hello".to_string()).string()
            );
        }
    }

    mod token {
        use super::{ArgumentValue, IncorrectArgumentType};

        #[test]
        fn returns_string_when_value_is_a_string() {
            assert_eq!(
                Ok("token".to_string()),
                ArgumentValue::Token("token".to_string()).token()
            );
        }

        #[test]
        fn returns_error_when_value_is_a_string() {
            assert_eq!(
                Err(IncorrectArgumentType {
                    expected: "token".to_string(),
                    got: "integer".to_string(),
                }),
                ArgumentValue::Integer(7).token()
            );
        }

        #[test]
        fn returns_error_when_value_is_a_token() {
            assert_eq!(
                Err(IncorrectArgumentType {
                    expected: "token".to_string(),
                    got: "string".to_string(),
                }),
                ArgumentValue::String("hello".to_string()).token()
            );
        }
    }
}
