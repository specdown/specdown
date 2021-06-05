use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::streaming::{tag, take_until},
    character::streaming::{alpha1, alphanumeric1, digit1, space0},
    combinator::map,
    multi::{many0, separated_list},
    sequence::{delimited, tuple},
    IResult,
};

use super::argument_value::ArgumentValue;
use super::function::Function;

pub type Argument<'a> = (&'a str, ArgumentValue);

pub fn parse(input: &str) -> IResult<&str, Function> {
    let p = tuple((space0, alpha1, space0, argument_list));
    map(p, |(_, name, _, arguments)| Function::new(name, arguments))(input)
}

fn argument_list(input: &str) -> IResult<&str, HashMap<String, ArgumentValue>> {
    let p = delimited(
        tuple((tag("("), space0)),
        separated_list(tuple((space0, tag(","), space0)), argument),
        tuple((space0, tag(")"))),
    );

    map(p, |args| list_of_args_to_hash_map(&args))(input)
}

fn list_of_args_to_hash_map(arguments: &[Argument]) -> HashMap<String, ArgumentValue> {
    arguments
        .iter()
        .map(|(name, value)| (String::from(*name), value.clone()))
        .collect()
}

fn argument(input: &str) -> IResult<&str, Argument> {
    let p = tuple((
        argument_name,
        tuple((space0, tag("="), space0)),
        argument_value,
    ));
    map(p, |(name, _, value)| (name, value))(input)
}

fn argument_name(input: &str) -> IResult<&str, &str> {
    let p = tuple((alpha1, many0(alt((alphanumeric1, tag("_"))))));
    let (remainder, (start, parts)) = p(input)?;
    let length = start.len() + parts.join("").len();
    Ok((remainder, &input[0..length]))
}

fn argument_value(input: &str) -> IResult<&str, ArgumentValue> {
    alt((integer_value, string_value, token_value))(input)
}

fn integer_value(input: &str) -> IResult<&str, ArgumentValue> {
    let p = digit1;
    map(p, |s: &str| ArgumentValue::Integer(s.parse().unwrap()))(input)
}

fn string_value(input: &str) -> IResult<&str, ArgumentValue> {
    let p = delimited(tag("\""), take_until("\""), tag("\""));
    map(p, |s: &str| ArgumentValue::String(s.to_string()))(input)
}

fn token_value(input: &str) -> IResult<&str, ArgumentValue> {
    map(alpha1, |token: &str| {
        ArgumentValue::Token(token.to_string())
    })(input)
}

#[cfg(test)]
mod tests {
    use super::{argument, argument_list, argument_value, parse, ArgumentValue, Function, HashMap};

    mod parse {
        use super::{parse, ArgumentValue, Function, HashMap};
        use maplit::hashmap;

        #[test]
        fn succeeds_when_function_has_no_args() {
            assert_eq!(
                parse("func(), more"),
                Ok((
                    ", more",
                    Function {
                        name: "func".to_string(),
                        arguments: HashMap::new(),
                    }
                ))
            );
        }

        #[test]
        fn succeeds_when_function_has_leading_whitespace() {
            assert_eq!(
                parse(" func(), more"),
                Ok((
                    ", more",
                    Function {
                        name: "func".to_string(),
                        arguments: HashMap::new(),
                    }
                ))
            );
        }

        #[test]
        fn succeeds_when_function_has_whitespace_before_opening_parenthesis() {
            assert_eq!(
                parse("func (), more"),
                Ok((
                    ", more",
                    Function {
                        name: "func".to_string(),
                        arguments: HashMap::new(),
                    }
                ))
            );
        }

        #[test]
        fn succeeds_when_function_has_args() {
            assert_eq!(
                parse("funcy(arg=\"hi\")"),
                Ok((
                    "",
                    Function {
                        name: "funcy".to_string(),
                        arguments: [("arg".to_string(), ArgumentValue::String("hi".to_string()))]
                            .iter()
                            .cloned()
                            .collect(),
                    }
                ))
            );
        }

        #[test]
        fn succeeds_when_no_arguments_are_provided() {
            let expected_fn = Function {
                name: "fn".to_string(),
                arguments: HashMap::new(),
            };
            assert_eq!(parse("fn()"), Ok(("", expected_fn)));
        }

        #[test]
        fn succeeds_when_one_argument_is_provided() {
            let expected_fn = Function {
                name: "fn".to_string(),
                arguments: [("arg".to_string(), ArgumentValue::String("abc".to_string()))]
                    .iter()
                    .cloned()
                    .collect(),
            };

            assert_eq!(parse("fn(arg=\"abc\")"), Ok(("", expected_fn)));
        }

        #[test]
        fn succeeds_when_multiple_arguments_are_provided() {
            let expected_fn = Function::new(
                "fn",
                hashmap! {
                    "arg1".to_string() => ArgumentValue::Token("abc".to_string()),
                    "arg2".to_string() => ArgumentValue::String("def".to_string()),
                },
            );

            assert_eq!(parse("fn(arg1=abc,arg2=\"def\")"), Ok(("", expected_fn)));
        }

        #[test]
        fn succeeds_when_multiple_arguments_are_provided_with_spaces() {
            let expected_fn = Function::new(
                "fn",
                hashmap! {
                        "arg1".to_string() => ArgumentValue::Token("xxx".to_string()),
                        "arg2".to_string() => ArgumentValue::String("123".to_string()),
                },
            );

            assert_eq!(parse("fn(arg1=xxx , arg2=\"123\")"), Ok(("", expected_fn)));
        }
    }

    mod argument_list {
        use maplit::hashmap;

        use super::{argument_list, ArgumentValue};

        #[test]
        fn succeeds_when_no_arguments_are_provided() {
            let expected_args = hashmap! {};
            assert_eq!(argument_list("()"), Ok(("", expected_args)));
        }

        #[test]
        fn succeeds_when_one_argument_is_provided() {
            let expected_args = hashmap! {
            "arg".to_string() => ArgumentValue::String("abc".to_string())
            };

            assert_eq!(argument_list("(arg=\"abc\")"), Ok(("", expected_args)));
        }

        #[test]
        fn succeeds_when_multiple_arguments_are_provided() {
            let expected_args = hashmap! {
                "arg1".to_string() => ArgumentValue::Token("abc".to_string()),
                "arg2".to_string() => ArgumentValue::String("def".to_string()),
            };
            assert_eq!(
                argument_list("(arg1=abc,arg2=\"def\")"),
                Ok(("", expected_args))
            );
        }

        #[test]
        fn succeeds_when_multiple_arguments_are_provided_with_spaces() {
            let expected_args = hashmap! {
                "arg1".to_string() => ArgumentValue::Token("xxx".to_string()),
                "arg2".to_string() => ArgumentValue::String("123".to_string()),
            };
            assert_eq!(
                argument_list("(arg1=xxx , arg2=\"123\")"),
                Ok(("", expected_args))
            );
        }

        #[test]
        fn succeeds_when_there_are_spaces_around_arguments() {
            let expected_args = hashmap! {
                "arg1".to_string() => ArgumentValue::Token("xxx".to_string()),
                "arg2".to_string() => ArgumentValue::String("123".to_string()),
            };
            assert_eq!(
                argument_list("(  arg1=xxx,arg2=\"123\"  )"),
                Ok(("", expected_args))
            );
        }
    }

    mod argument {
        use super::{argument, ArgumentValue};

        #[test]
        fn fails_when_name_starts_with_a_digit() {
            assert_eq!(
                argument("1arg=\"value\",more..."),
                Err(nom::Err::Error(nom::error_position!(
                    "1arg=\"value\",more...",
                    nom::error::ErrorKind::Alpha
                )))
            );
        }

        #[test]
        fn succeeds_when_single_alpha_char_name() {
            assert_eq!(
                argument("a=\"value\",more..."),
                Ok((
                    ",more...",
                    ("a", ArgumentValue::String("value".to_string()))
                ))
            );
        }

        #[test]
        fn succeeds_when_alpha_numeric_name() {
            assert_eq!(
                argument("arg1=\"value\",more..."),
                Ok((
                    ",more...",
                    ("arg1", ArgumentValue::String("value".to_string()))
                ))
            );
        }

        #[test]
        fn succeeds_with_token_argument() {
            assert_eq!(
                argument("arg=token,more..."),
                Ok((
                    ",more...",
                    ("arg", ArgumentValue::Token("token".to_string()))
                ))
            );
        }

        #[test]
        fn succeeds_when_arg_contains_underscore() {
            assert_eq!(
                argument("arg_name=\"value\",more..."),
                Ok((
                    ",more...",
                    ("arg_name", ArgumentValue::String("value".to_string()))
                ))
            );
        }

        #[test]
        fn succeeds_with_spaces_around_equals() {
            assert_eq!(
                argument("arg  =  token rest"),
                Ok((" rest", ("arg", ArgumentValue::Token("token".to_string()))))
            );
        }
    }

    mod argument_value {
        use super::{argument_value, ArgumentValue};

        #[test]
        fn succeeds_when_there_is_a_remainder() {
            assert_eq!(
                argument_value("\"string\" rest"),
                Ok((" rest", ArgumentValue::String("string".to_string())))
            );
        }

        // #[test]
        // fn succeeds_when_there_is_no_remainder() {
        //     let result = argument_value("token");
        //     assert_eq!(result, Ok(("", ArgumentValue::Token("token"))));
        // }

        mod integer_value {
            use super::{argument_value, ArgumentValue};

            #[test]
            fn succeeds_when_there_is_a_remainder() {
                assert_eq!(
                    argument_value("123 leftovers"),
                    Ok((" leftovers", ArgumentValue::Integer(123)))
                );
            }
        }

        mod string_value {
            use super::{argument_value, ArgumentValue};

            #[test]
            fn succeeds_when_there_is_a_remainder() {
                assert_eq!(
                    argument_value("\"arg_value1\" leftovers"),
                    Ok((
                        " leftovers",
                        ArgumentValue::String("arg_value1".to_string())
                    ))
                );
            }

            #[test]
            fn succeeds_when_there_is_no_remainder() {
                assert_eq!(
                    argument_value("\"arg_value2\""),
                    Ok(("", ArgumentValue::String("arg_value2".to_string())))
                );
            }

            #[test]
            fn fails_when_there_is_no_closing_quote() {
                assert_eq!(
                    argument_value("\"arg_value2"),
                    Err(nom::Err::Incomplete(nom::Needed::Size(1)))
                );
            }
        }

        mod token_value {
            use super::{argument_value, ArgumentValue};

            #[test]
            fn succeeds_when_there_is_a_remainder() {
                let result = argument_value("stdout leftovers");
                assert_eq!(
                    result,
                    Ok((" leftovers", ArgumentValue::Token("stdout".to_string())))
                );
            }

            // #[test]
            // fn succeeds_when_there_is_no_remainder() {
            //     let result = argument_value("stderr");
            //     assert_eq!(result, Ok(("", ArgumentValue::Token("stderr"))));
            // }
        }
    }
}
