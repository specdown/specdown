use crate::results::test_result::TestResult;
use crate::runner::state::ScriptOutput;
use crate::types::{ScriptName, Source, Stream, VerifyValue};

use super::error::Error;

pub fn run(
    source: &Source,
    value: &VerifyValue,
    script_output: &dyn ScriptOutput,
) -> Result<TestResult, Error> {
    let Source {
        name: ScriptName(script_name),
        stream,
    } = source;
    let VerifyValue(value_string) = value;

    let got_raw = match stream {
        Stream::StdOut => script_output
            .get_stdout(script_name)
            .expect("failed to get script stdout"),
        Stream::StdErr => script_output
            .get_stderr(script_name)
            .expect("failed to get script stderr"),
    };

    let expected = strip_ansi_escape_chars(value_string);
    let got = strip_ansi_escape_chars(got_raw);
    let success = expected == got;

    let result = TestResult::Verify {
        script_name: script_name.to_string(),
        stream: stream_to_string(stream).into(),
        expected,
        got,
        success,
    };

    Ok(result)
}

fn stream_to_string(stream: &Stream) -> &str {
    match stream {
        Stream::StdOut => "stdout",
        Stream::StdErr => "stderr",
    }
}

fn strip_ansi_escape_chars(string: &str) -> String {
    strip_ansi_escapes::strip(string)
        .expect("ANSI code to be stripped from got")
        .iter()
        .map(|&c| c as char)
        .collect()
}

#[cfg(test)]
mod tests {

}