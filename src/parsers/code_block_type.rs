use crate::parsers::error::{Error, Result};
use crate::parsers::function_string_parser;
use crate::parsers::function_string_parser::Function;
use crate::types::{
    DelayMillis, ExitCode, FilePath, MockName, OutputExpectation, ResponseBody, ResponseCodeBlock,
    ScriptName, Source, StatusCode, Stream, TargetOs,
};
use nom::combinator::map_res;
use nom::{IResult, Parser};

#[derive(Debug, Eq, PartialEq)]
pub struct ScriptCodeBlock {
    pub script_name: Option<ScriptName>,
    pub expected_exit_code: Option<ExitCode>,
    pub expected_output: OutputExpectation,
}

#[derive(Debug, Eq, PartialEq)]
pub struct VerifyCodeBlock {
    pub source: Source,
    pub target_os: Option<TargetOs>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct BackgroundCodeBlock {
    pub script_name: Option<ScriptName>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum CodeBlockType {
    Script(ScriptCodeBlock),
    Verify(VerifyCodeBlock),
    CreateFile(FilePath),
    Background(BackgroundCodeBlock),
    Response(ResponseCodeBlock),
    Skip(),
}

pub fn parse(input: &str) -> IResult<&str, CodeBlockType, Error> {
    map_res(function_string_parser::parse, from_function).parse(input)
}

fn from_function(f: Function) -> Result<CodeBlockType> {
    match &f.name[..] {
        "script" => script_to_code_block_type(&f),
        "verify" => verify_to_code_block_type(&f),
        "file" => file_to_code_block_type(&f),
        "background" => background_to_code_block_type(&f),
        "response" => response_to_code_block_type(&f),
        "skip" => Ok(skip_to_code_block_type(&f)),
        _ => Err(Error::UnknownFunction(f.name)),
    }
}

fn script_to_code_block_type(f: &Function) -> Result<CodeBlockType> {
    let name = if f.has_argument("name") {
        Some(ScriptName(f.get_string_argument("name")?))
    } else {
        None
    };
    let expected_exit_code = if f.has_argument("expected_exit_code") {
        Some(ExitCode(f.get_integer_argument("expected_exit_code")?))
    } else {
        None
    };
    let expected_output = f
        .get_token_argument("expected_output")
        .or_else(|_| Ok("any".to_string()))
        .and_then(|s| to_expected_output(&s))?;
    Ok(CodeBlockType::Script(ScriptCodeBlock {
        script_name: name,
        expected_exit_code,
        expected_output,
    }))
}

fn to_expected_output(s: &str) -> Result<OutputExpectation> {
    match s {
        "any" => Ok(OutputExpectation::Any),
        "stdout" => Ok(OutputExpectation::StdOut),
        "stderr" => Ok(OutputExpectation::StdErr),
        "none" => Ok(OutputExpectation::None),
        _ => Err(Error::InvalidArgumentValue {
            function: "script".to_string(),
            argument: "expected_output".to_string(),
            expected: "any, stdout, stderr or none".to_string(),
            got: s.to_string(),
        }),
    }
}

fn file_to_code_block_type(f: &Function) -> Result<CodeBlockType> {
    let path = f.get_string_argument("path")?;
    Ok(CodeBlockType::CreateFile(FilePath(path)))
}

fn background_to_code_block_type(f: &Function) -> Result<CodeBlockType> {
    let name = if f.has_argument("name") {
        Some(ScriptName(f.get_string_argument("name")?))
    } else {
        None
    };
    Ok(CodeBlockType::Background(BackgroundCodeBlock {
        script_name: name,
    }))
}

fn response_to_code_block_type(f: &Function) -> Result<CodeBlockType> {
    let name = MockName(f.get_string_argument("name")?);

    let status = if f.has_argument("status") {
        let raw = f.get_integer_argument("status")?;
        StatusCode::parse(raw).map_err(|_| Error::InvalidArgumentValue {
            function: "response".to_string(),
            argument: "status".to_string(),
            expected: "an HTTP status code between 100 and 599".to_string(),
            got: raw.to_string(),
        })?
    } else {
        StatusCode::default()
    };

    let headers = if f.has_argument("headers") {
        Some(f.get_string_argument("headers")?)
    } else {
        None
    };

    let content_type = if f.has_argument("content_type") {
        Some(f.get_string_argument("content_type")?)
    } else {
        None
    };

    let delay = if f.has_argument("delay") {
        let raw = f.get_integer_argument("delay")?;
        DelayMillis::parse(raw).map_err(|_| Error::InvalidArgumentValue {
            function: "response".to_string(),
            argument: "delay".to_string(),
            expected: "a delay in milliseconds between 0 and 300000".to_string(),
            got: raw.to_string(),
        })?
    } else {
        DelayMillis::default()
    };

    let body = if f.has_argument("body") {
        ResponseBody::Inline(f.get_string_argument("body")?)
    } else {
        ResponseBody::Empty
    };

    Ok(CodeBlockType::Response(ResponseCodeBlock {
        name,
        status,
        headers,
        content_type,
        delay,
        body,
    }))
}

const fn skip_to_code_block_type(_f: &Function) -> CodeBlockType {
    CodeBlockType::Skip()
}

fn verify_to_code_block_type(f: &Function) -> Result<CodeBlockType> {
    let name = if f.has_argument("script_name") {
        Some(ScriptName(f.get_string_argument("script_name")?))
    } else {
        None
    };
    let stream_name = if f.has_argument("stream") {
        f.get_token_argument("stream")?
    } else {
        "stdout".to_string()
    };
    let target_os = if f.has_argument("target_os") {
        Some(TargetOs(f.get_string_argument("target_os")?))
    } else {
        None
    };
    let stream = to_stream(&stream_name).ok_or_else(|| Error::InvalidArgumentValue {
        function: f.name.clone(),
        argument: "stream".to_string(),
        got: stream_name.clone(),
        expected: "output, stdout or stderr".to_string(),
    })?;
    Ok(CodeBlockType::Verify(VerifyCodeBlock {
        source: Source { name, stream },
        target_os,
    }))
}

fn to_stream(stream_name: &str) -> Option<Stream> {
    match stream_name {
        "stdout" => Some(Stream::StdOut),
        "stderr" => Some(Stream::StdErr),
        _ => None,
    }
}
