use std::collections::HashMap;

use super::error;

use super::argument_value::{ArgumentValue, IncorrectArgumentType};

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub arguments: HashMap<String, ArgumentValue>,
}

impl Function {
    pub fn has_argument(&self, name: &str) -> bool {
        self.arguments.contains_key(name)
    }

    pub fn get_integer_argument(&self, name: &str) -> error::Result<i32> {
        self.get_required_argument(name)?
            .integer()
            .map_err(|err| self.incorrect_argument_type_error(name, err))
    }

    pub fn get_string_argument(&self, name: &str) -> error::Result<String> {
        self.get_required_argument(name)?
            .string()
            .map_err(|err| self.incorrect_argument_type_error(name, err))
    }

    pub fn get_token_argument(&self, name: &str) -> error::Result<String> {
        self.get_required_argument(name)?
            .token()
            .map_err(|err| self.incorrect_argument_type_error(name, err))
    }

    fn get_required_argument<'a>(&'a self, name: &str) -> error::Result<&'a ArgumentValue> {
        self.arguments
            .get(name)
            .ok_or_else(|| error::Error::MissingArgument {
                function: self.name.clone(),
                argument: name.to_string(),
            })
    }

    fn incorrect_argument_type_error(
        &self,
        argument: &str,
        IncorrectArgumentType { expected, got }: IncorrectArgumentType,
    ) -> error::Error {
        error::Error::IncorrectArgumentType {
            function: self.name.to_string(),
            argument: argument.to_string(),
            expected,
            got,
        }
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
