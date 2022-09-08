use nom::bytes::streaming::{tag, take_until};
use nom::combinator::map;
use nom::error::ParseError;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

#[derive(Debug, Eq, PartialEq)]
pub struct CodeBlockInfo<Extra> {
    pub language: String,
    pub extra: Extra,
}

pub fn parse<'a, Output, Error: ParseError<&'a str>, ExtraInfoParser>(
    extra_info_parser: ExtraInfoParser,
) -> impl FnMut(&'a str) -> IResult<&'a str, CodeBlockInfo<Output>, Error>
where
    ExtraInfoParser: Parser<&'a str, Output, Error>,
{
    map(
        separated_pair(take_until(","), tag(","), extra_info_parser),
        |(language, extra)| CodeBlockInfo {
            language: language.to_string(),
            extra,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{parse, CodeBlockInfo};

    mod parse {
        use super::{parse, CodeBlockInfo};
        use nom::bytes::complete::tag;
        use nom::combinator::rest;
        use nom::error::ErrorKind::Tag;
        use nom::IResult;

        #[test]
        fn successful_parsing_with_a_rest_parser() {
            let result: IResult<&str, CodeBlockInfo<&str>, nom::error::Error<&str>> =
                parse(rest)("rust,remaining");
            assert_eq!(
                result,
                Ok((
                    "",
                    CodeBlockInfo {
                        language: "rust".to_string(),
                        extra: "remaining"
                    }
                ))
            );
        }

        #[test]
        fn failing_parsing_when_no_comma() {
            let result: IResult<&str, CodeBlockInfo<&str>, nom::error::Error<&str>> =
                parse(rest)("rust");
            assert_eq!(result, Err(nom::Err::Incomplete(nom::Needed::Unknown)));
        }

        #[test]
        fn failing_extra_info_parser_fails() {
            let result: IResult<&str, CodeBlockInfo<&str>, nom::error::Error<&str>> =
                parse(tag("x"))("rust,y");
            assert_eq!(
                result,
                Err(nom::Err::Error(nom::error::Error {
                    input: "y",
                    code: Tag
                }))
            );
        }
    }
}
