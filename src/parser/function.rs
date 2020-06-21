use std::collections::HashMap;

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
