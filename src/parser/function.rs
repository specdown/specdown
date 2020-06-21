use std::collections::HashMap;

use super::error::{Error, Result};

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub arguments: HashMap<String, ArgumentValue>,
}

pub type Argument<'a> = (&'a str, ArgumentValue);

#[derive(Debug, PartialEq, Clone)]
pub enum ArgumentValue {
    Integer(u32),
    String(String),
    Token(String),
}

impl Function {
    pub fn has_argument(&self, name: &str) -> bool {
        self.arguments.contains_key(name)
    }

    pub fn get_integer_argument(&self, name: &str) -> Result<u32> {
        match self.get_required_argument(name)? {
            ArgumentValue::Integer(num) => Ok(*num),
            ArgumentValue::String(_) => {
                self.incorrect_argument_type_error(name, "integer", "string")
            }
            ArgumentValue::Token(_) => self.incorrect_argument_type_error(name, "integer", "token"),
        }
    }

    pub fn get_string_argument(&self, name: &str) -> Result<String> {
        match self.get_required_argument(name)? {
            ArgumentValue::String(s) => Ok(s.clone()),
            ArgumentValue::Integer(_) => {
                self.incorrect_argument_type_error(name, "string", "integer")
            }
            ArgumentValue::Token(_) => self.incorrect_argument_type_error(name, "string", "token"),
        }
    }

    pub fn get_token_argument(&self, name: &str) -> Result<String> {
        match self.get_required_argument(name)? {
            ArgumentValue::Token(t) => Ok(t.clone()),
            ArgumentValue::Integer(_) => {
                self.incorrect_argument_type_error(name, "token", "integer")
            }
            ArgumentValue::String(_) => self.incorrect_argument_type_error(name, "token", "string"),
        }
    }

    fn get_required_argument<'a>(&'a self, name: &str) -> Result<&'a ArgumentValue> {
        self.arguments
            .get(name)
            .ok_or_else(|| Error::MissingArgument {
                function: self.name.clone(),
                argument: name.to_string(),
            })
    }

    fn incorrect_argument_type_error<T>(
        &self,
        argument: &str,
        expected: &str,
        got: &str,
    ) -> Result<T> {
        Err(Error::IncorrectArgumentType {
            function: self.name.to_string(),
            argument: argument.to_string(),
            expected: expected.to_string(),
            got: got.to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::{ArgumentValue, Function};
    mod has_argument {
        use super::{ArgumentValue, Function};
        use maplit::hashmap;

        #[test]
        fn returns_true_when_argument_is_present() {
            let f = Function {
                name: "abc".to_string(),
                arguments: hashmap! {
                    "arg1".to_string() => ArgumentValue::Integer(1),
                    "arg2".to_string() => ArgumentValue::Integer(2),
                },
            };
            assert_eq!(true, f.has_argument("arg1"));
            assert_eq!(true, f.has_argument("arg2"));
        }

        #[test]
        fn returns_false_when_argument_is_present() {
            let f = Function {
                name: "abc".to_string(),
                arguments: hashmap! {
                    "arg".to_string() => ArgumentValue::Integer(1),
                },
            };
            assert_eq!(false, f.has_argument("not-arg"))
        }
    }
}
