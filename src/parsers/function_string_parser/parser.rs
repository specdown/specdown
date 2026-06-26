use std::collections::HashMap;

use nom::error::{ErrorKind, ParseError};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, alphanumeric1, digit1, space0},
    combinator::map,
    multi::{many0, separated_list0},
    sequence::delimited,
    IResult, Parser,
};

use super::argument_value::ArgumentValue;
use super::function::Function;

pub type Argument<'a> = (&'a str, ArgumentValue);

pub fn parse<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Function, E> {
    let p = (space0, alpha1, space0, argument_list);
    map(p, |(_, name, _, arguments)| Function::new(name, arguments)).parse(input)
}

fn argument_list<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, HashMap<String, ArgumentValue>, E> {
    let p = delimited(
        (tag("("), space0),
        separated_list0((space0, tag(","), space0), argument),
        (space0, tag(")")),
    );

    map(p, |args| list_of_args_to_hash_map(&args)).parse(input)
}

fn list_of_args_to_hash_map(arguments: &[Argument<'_>]) -> HashMap<String, ArgumentValue> {
    arguments
        .iter()
        .map(|(name, value)| (String::from(*name), value.clone()))
        .collect()
}

fn argument<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Argument<'a>, E> {
    let p = (argument_name, (space0, tag("="), space0), argument_value);
    map(p, |(name, _, value)| (name, value)).parse(input)
}

fn argument_name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut p = (alpha1, many0(alt((alphanumeric1, tag("_")))));
    let (remainder, (start, parts)) = p.parse(input)?;
    let length = start.len() + parts.iter().map(|p| (*p).len()).sum::<usize>();
    Ok((remainder, &input[0..length]))
}

fn argument_value<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ArgumentValue, E> {
    alt((integer_value, string_value, token_value)).parse(input)
}

fn integer_value<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ArgumentValue, E> {
    let (remainder, digits) = digit1::<&'a str, E>.parse(input)?;

    match digits.parse::<i32>() {
        Ok(value) => Ok((remainder, ArgumentValue::Integer(value))),
        Err(_) => Err(nom::Err::Error(E::from_error_kind(
            input,
            ErrorKind::MapRes,
        ))),
    }
}

fn string_value<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ArgumentValue, E> {
    let p = delimited(tag("\""), take_until("\""), tag("\""));
    map(p, |s: &'a str| ArgumentValue::String(s.to_string())).parse(input)
}

fn token_value<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ArgumentValue, E> {
    map(alpha1, |token: &'a str| {
        ArgumentValue::Token(token.to_string())
    })
    .parse(input)
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
                parse::<nom::error::Error<&str>>("func(), more"),
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
                parse::<nom::error::Error<&str>>(" func(), more"),
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
                parse::<nom::error::Error<&str>>("func (), more"),
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
                parse::<nom::error::Error<&str>>("funcy(arg=\"hi\")"),
                Ok((
                    "",
                    Function {
                        name: "funcy".to_string(),
                        arguments: std::iter::once(&(
                            "arg".to_string(),
                            ArgumentValue::String("hi".to_string())
                        ))
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
            assert_eq!(
                parse::<nom::error::Error<&str>>("fn()"),
                Ok(("", expected_fn))
            );
        }

        #[test]
        fn succeeds_when_one_argument_is_provided() {
            let expected_fn = Function {
                name: "fn".to_string(),
                arguments: std::iter::once(&(
                    "arg".to_string(),
                    ArgumentValue::String("abc".to_string()),
                ))
                .cloned()
                .collect(),
            };

            assert_eq!(
                parse::<nom::error::Error<&str>>("fn(arg=\"abc\")"),
                Ok(("", expected_fn))
            );
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

            assert_eq!(
                parse::<nom::error::Error<&str>>("fn(arg1=abc,arg2=\"def\")"),
                Ok(("", expected_fn))
            );
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

            assert_eq!(
                parse::<nom::error::Error<&str>>("fn(arg1=xxx , arg2=\"123\")"),
                Ok(("", expected_fn))
            );
        }
    }

    mod argument_list {
        use maplit::hashmap;

        use super::{argument_list, ArgumentValue};

        #[test]
        fn succeeds_when_no_arguments_are_provided() {
            let expected_args = hashmap! {};
            assert_eq!(
                argument_list::<nom::error::Error<&str>>("()"),
                Ok(("", expected_args))
            );
        }

        #[test]
        fn succeeds_when_one_argument_is_provided() {
            let expected_args = hashmap! {
            "arg".to_string() => ArgumentValue::String("abc".to_string())
            };

            assert_eq!(
                argument_list::<nom::error::Error<&str>>("(arg=\"abc\")"),
                Ok(("", expected_args))
            );
        }

        #[test]
        fn succeeds_when_multiple_arguments_are_provided() {
            let expected_args = hashmap! {
                "arg1".to_string() => ArgumentValue::Token("abc".to_string()),
                "arg2".to_string() => ArgumentValue::String("def".to_string()),
            };
            assert_eq!(
                argument_list::<nom::error::Error<&str>>("(arg1=abc,arg2=\"def\")"),
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
                argument_list::<nom::error::Error<&str>>("(arg1=xxx , arg2=\"123\")"),
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
                argument_list::<nom::error::Error<&str>>("(  arg1=xxx,arg2=\"123\"  )"),
                Ok(("", expected_args))
            );
        }
    }

    mod argument {
        use super::{argument, ArgumentValue};

        #[test]
        fn fails_when_name_starts_with_a_digit() {
            assert_eq!(
                argument::<nom::error::Error<&str>>("1arg=\"value\",more..."),
                Err(nom::Err::Error(nom::error_position!(
                    "1arg=\"value\",more...",
                    nom::error::ErrorKind::Alpha
                )))
            );
        }

        #[test]
        fn succeeds_when_single_alpha_char_name() {
            assert_eq!(
                argument::<nom::error::Error<&str>>("a=\"value\",more..."),
                Ok((
                    ",more...",
                    ("a", ArgumentValue::String("value".to_string()))
                ))
            );
        }

        #[test]
        fn succeeds_when_alpha_numeric_name() {
            assert_eq!(
                argument::<nom::error::Error<&str>>("arg1=\"value\",more..."),
                Ok((
                    ",more...",
                    ("arg1", ArgumentValue::String("value".to_string()))
                ))
            );
        }

        #[test]
        fn succeeds_with_token_argument() {
            assert_eq!(
                argument::<nom::error::Error<&str>>("arg=token,more..."),
                Ok((
                    ",more...",
                    ("arg", ArgumentValue::Token("token".to_string()))
                ))
            );
        }

        #[test]
        fn succeeds_when_arg_contains_underscore() {
            assert_eq!(
                argument::<nom::error::Error<&str>>("arg_name=\"value\",more..."),
                Ok((
                    ",more...",
                    ("arg_name", ArgumentValue::String("value".to_string()))
                ))
            );
        }

        #[test]
        fn succeeds_with_spaces_around_equals() {
            assert_eq!(
                argument::<nom::error::Error<&str>>("arg  =  token rest"),
                Ok((" rest", ("arg", ArgumentValue::Token("token".to_string()))))
            );
        }
    }

    mod argument_value {
        use super::{argument_value, ArgumentValue};

        #[test]
        fn succeeds_when_there_is_a_remainder() {
            assert_eq!(
                argument_value::<nom::error::Error<&str>>("\"string\" rest"),
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
                    argument_value::<nom::error::Error<&str>>("123 leftovers"),
                    Ok((" leftovers", ArgumentValue::Integer(123)))
                );
            }
        }

        mod string_value {
            use super::{argument_value, ArgumentValue};

            #[test]
            fn succeeds_when_there_is_a_remainder() {
                assert_eq!(
                    argument_value::<nom::error::Error<&str>>("\"arg_value1\" leftovers"),
                    Ok((
                        " leftovers",
                        ArgumentValue::String("arg_value1".to_string())
                    ))
                );
            }

            #[test]
            fn succeeds_when_there_is_no_remainder() {
                assert_eq!(
                    argument_value::<nom::error::Error<&str>>("\"arg_value2\""),
                    Ok(("", ArgumentValue::String("arg_value2".to_string())))
                );
            }

            #[test]
            fn fails_when_there_is_no_closing_quote() {
                assert_eq!(
                    argument_value::<nom::error::Error<&str>>("\"arg_value2"),
                    Err(nom::Err::Error(nom::error::Error {
                        input: "\"arg_value2",
                        code: nom::error::ErrorKind::Alpha
                    }))
                );
            }
        }

        mod token_value {
            use super::{argument_value, ArgumentValue};

            #[test]
            fn succeeds_when_there_is_a_remainder() {
                let result = argument_value::<nom::error::Error<&str>>("stdout leftovers");
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

    mod quickcheck_properties {
        use std::convert::TryFrom;

        use quickcheck::TestResult;
        use quickcheck_macros::quickcheck;

        use super::{parse, ArgumentValue};

        /// A valid in-range integer argument round-trips to the expected value;
        /// an out-of-range integer is rejected without panicking.
        #[quickcheck]
        fn round_trips_a_valid_integer_argument(value: u32) -> TestResult {
            let input = format!("fn(n={value})");
            match i32::try_from(value) {
                Ok(expected) => match parse::<nom::error::Error<&str>>(&input) {
                    Ok((_, f)) => TestResult::from_bool(
                        f.arguments.get("n") == Some(&ArgumentValue::Integer(expected)),
                    ),
                    Err(_) => TestResult::failed(),
                },
                Err(_) => TestResult::from_bool(parse::<nom::error::Error<&str>>(&input).is_err()),
            }
        }
    }

    mod adversarial_inputs {
        use super::parse;

        /// Adversarial inputs that previously caused panics or crashes.
        /// Every info string flows through this parser, so none may abort the process.
        #[test]
        fn never_panics_on_known_bad_inputs() {
            let bad_inputs = [
                "99999999999999999999)",
                "\u{0}",
                "",
                "\"",
                "fn(n=\"unterminated",
                "(((((",
                "a=b=c=d=e=f=g",
                "fn(=)",
                "fn(=123)",
            ];
            for input in bad_inputs {
                let _ = parse::<nom::error::Error<&str>>(input);
            }
        }
    }
}
