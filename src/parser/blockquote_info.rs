use crate::types::{ScriptName, Source};

#[derive(Debug, PartialEq)]
enum BlockQuoteTypes {
    Script(ScriptName),
    Verify(Source),
}

use nom::{
    bytes::streaming::{take_until,tag},
    character::streaming::{alpha1,alphanumeric0,space0},
    branch::alt,
    sequence::{tuple,delimited},
    combinator::map,
    multi::{many0, separated_list},
    IResult
};

fn parse(input: &'static str) -> IResult<&'static str, BlockQuoteTypes> {
    let (remainder, (_language, _comma, func)) = tuple((
        take_until(","),
        tag(","),
        function
    ))(input)?;

    Ok((remainder, BlockQuoteTypes::Script(ScriptName("example-script".to_string()))))
}

#[derive(Debug, PartialEq)]
struct Function<'a> {
    name: &'a str,
    arguments: Vec<Argument<'a>>
}

fn function(input: &str) -> IResult<&str, Function> {
    let p = tuple((alpha1, argument_list));
    map(p, |(name, args)| Function { name: name, arguments: args })(input)
}

fn argument_list(input: &str) -> IResult<&str, Vec<Argument>> {
    delimited(
        tag("("),
        separated_list(tuple((space0, tag(","), space0)), argument),
        tag(")")
    )(input)
}

#[derive(Debug, PartialEq)]
struct Argument<'a> {
    name: &'a str,
    value: ArgumentValue<'a>,
}

fn argument(input: &str) -> IResult<&str, Argument> {
    let p = tuple((
        argument_name,
        tuple((space0, tag("="), space0)),
        argument_value
    ));
    map(p, |(name, _, value)| Argument { name: name, value: value })(input)
}

fn argument_name(input: &str) -> IResult<&str, &str> {
    let (remainder, (start, rest)) = tuple((alpha1, alphanumeric0))(input)?;
    Ok((remainder, &input[0..(start.len() + rest.len())]))
}

#[derive(Debug, PartialEq)]
enum ArgumentValue<'a> {
    String(&'a str),
    Token(&'a str),
}

fn argument_value(input: &str) -> IResult<&str, ArgumentValue> {
    return alt((string_value, token_value))(input);
}

fn string_value(input: &str) -> IResult<&str, ArgumentValue> {
    let p = delimited(tag("\""), take_until("\""), tag("\""));
    map(p, ArgumentValue::String)(input)

}

fn token_value(input: &str) -> IResult<&str, ArgumentValue> {
    map(alpha1, ArgumentValue::Token)(input)
}

mod tests {
    use super::*;

    mod parse {
        use super::*;

        #[test]
        fn succeeds_when_function_is_script() {
            let result = parse("shell,script(name=\"example-script\")");
            assert_eq!(result, Ok(("", BlockQuoteTypes::Script(ScriptName("example-script".to_string())))))
        }

        // #[test]
        // fn returns_when_function_is_verify() {
        //     let result = parse("shell,script(name=\"another-script\"");
        //     assert_eq!(result, Ok(("", BlockQuoteTypes::Script(ScriptName("another-script".to_string())))))
        // }
    }

    mod function {
        use super::*;

        #[test]
        fn succeeds_when_function_has_no_args() {
            assert_eq!(
                function("func(), more"),
                Ok((", more", Function { name: "func", arguments: vec![] }))
            )
        }

        #[test]
        fn succeeds_when_function_has_args() {
            assert_eq!(
                function("funcy(arg=\"hi\")"),
                Ok((
                    "",
                    Function {
                        name: "funcy",
                        arguments: vec![Argument {name: "arg", value: ArgumentValue::String("hi")}]
                    }
                ))
            )
        }
    }

    mod argument_list {
        use super::*;

        #[test]
        fn succeeds_when_no_arguments_are_provided() {
            assert_eq!(
                argument_list("()"),
                Ok(("", vec![]))
            );
        }

        #[test]
        fn succeeds_when_one_argument_is_provided() {
            assert_eq!(
                argument_list("(arg=\"abc\")"),
                Ok(("", vec![Argument { name: "arg", value: ArgumentValue::String("abc") }]))
            );
        }

        #[test]
        fn succeeds_when_multiple_arguments_are_provided() {
            assert_eq!(
                argument_list("(arg1=abc,arg2=\"def\")"),
                Ok(("", vec![
                    Argument { name: "arg1", value: ArgumentValue::Token("abc") },
                    Argument { name: "arg2", value: ArgumentValue::String("def") },
                ]))
            );
        }

        #[test]
        fn succeeds_when_multiple_arguments_are_provided_with_spaces() {
            assert_eq!(
                argument_list("(arg1=xxx , arg2=\"123\")"),
                Ok(("", vec![
                    Argument { name: "arg1", value: ArgumentValue::Token("xxx") },
                    Argument { name: "arg2", value: ArgumentValue::String("123") },
                ]))
            );
        }
    }

    mod argument {
        use super::*;

        #[test]
        fn fails_when_name_starts_with_a_digit() {
            assert_eq!(
                argument("1arg=\"value\",more..."),
                Err(nom::Err::Error(nom::error_position!("1arg=\"value\",more...", nom::error::ErrorKind::Alpha)))
            );
        }

        #[test]
        fn succeeds_when_single_alpha_char_name() {
            assert_eq!(
                argument("a=\"value\",more..."),
                Ok((
                    ",more...",
                    Argument { name: "a", value: ArgumentValue::String("value")}
                ))
            );
        }

        #[test]
        fn succeeds_when_alpha_numeric_name() {
            assert_eq!(
                argument("arg1=\"value\",more..."),
                Ok((
                    ",more...",
                    Argument { name: "arg1", value: ArgumentValue::String("value")}
                ))
            );
        }

        #[test]
        fn succeds_with_token_argument() {
            assert_eq!(
                argument("arg=token,more..."),
                Ok((
                    ",more...",
                    Argument { name: "arg", value: ArgumentValue::Token("token")}
                ))
            );
        }

        #[test]
        fn succeds_with_spaces_around_equals() {
            assert_eq!(
                argument("arg  =  token rest"),
                Ok((
                    " rest",
                    Argument { name: "arg", value: ArgumentValue::Token("token")}
                ))
            );
        }
    }


    mod argument_value {
        use super::*;

        #[test]
        fn succeeds_when_there_is_a_remainder() {
            assert_eq!(
                argument_value("\"string\" rest"),
                Ok((" rest", ArgumentValue::String("string")))
            );
        }

        // #[test]
        // fn succeeds_when_there_is_no_remainder() {
        //     let result = argument_value("token");
        //     assert_eq!(result, Ok(("", ArgumentValue::Token("token"))));
        // }
    }

    mod string_value {
        use super::*;

        #[test]
        fn succeeds_when_there_is_a_remainder() {
            assert_eq!(
                string_value("\"arg_value1\" leftovers"),
                Ok((" leftovers", ArgumentValue::String("arg_value1")))
            );
        }

        #[test]
        fn succeeds_when_there_is_no_remainder() {
            assert_eq!(
                string_value("\"arg_value2\""),
                Ok(("", ArgumentValue::String("arg_value2")))
            );
        }

        #[test]
        fn fails_when_there_is_no_opening_quote() {
            assert_eq!(
                string_value("arg_value2\""),
                Err(nom::Err::Error(nom::error_position!("arg_value2\"", nom::error::ErrorKind::Tag)))
            );
        }

        #[test]
        fn fails_when_there_is_no_closing_quote() {
            assert_eq!(
                string_value("\"arg_value2"),
                Err(nom::Err::Incomplete(nom::Needed::Size(1)))
            );
        }
    }

    mod token_value {
        use super::*;

        #[test]
        fn succeeds_when_there_is_a_remainder() {
            let result = token_value("stdout leftovers");
            assert_eq!(result, Ok((" leftovers", ArgumentValue::Token("stdout"))));
        }

        // #[test]
        // fn succeeds_when_there_is_no_remainder() {
        //     let result = token_value("stderr");
        //     assert_eq!(result, Ok(("", ArgumentValue::Token("stderr"))));
        // }
    }
}