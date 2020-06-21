#[derive(Debug, PartialEq, Clone)]
pub enum ArgumentValue {
    Integer(u32),
    String(String),
    Token(String),
}

pub struct IncorrectArgumentType {
    pub expected: String,
    pub got: String,
}

impl ArgumentValue {
    pub fn integer(&self) -> Result<u32, IncorrectArgumentType> {
        match self {
            ArgumentValue::Integer(num) => Ok(*num),
            ArgumentValue::String(_) => Self::incorrect_argument_type_error("integer", "string"),
            ArgumentValue::Token(_) => Self::incorrect_argument_type_error("integer", "token"),
        }
    }

    pub fn string(&self) -> Result<String, IncorrectArgumentType> {
        match self {
            ArgumentValue::String(s) => Ok(s.clone()),
            ArgumentValue::Integer(_) => Self::incorrect_argument_type_error("string", "integer"),
            ArgumentValue::Token(_) => Self::incorrect_argument_type_error("string", "token"),
        }
    }

    pub fn token(&self) -> Result<String, IncorrectArgumentType> {
        match self {
            ArgumentValue::Token(t) => Ok(t.clone()),
            ArgumentValue::Integer(_) => Self::incorrect_argument_type_error("token", "integer"),
            ArgumentValue::String(_) => Self::incorrect_argument_type_error("token", "string"),
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
mod test {}
