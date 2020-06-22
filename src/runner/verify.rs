use crate::results::test_result::TestResult;
use crate::runner::state::State;
use crate::types::{ScriptName, Source, Stream, VerifyValue};

use super::error::Error;

pub fn run(source: &Source, value: &VerifyValue, state: &mut State) -> Result<TestResult, Error> {
    let Source {
        name: ScriptName(script_name),
        stream,
    } = source;
    let VerifyValue(value_string) = value;

    let got = match stream {
        Stream::StdOut => state
            .get_script_stdout(script_name)
            .expect("failed to get script stdout"),
        Stream::StdErr => state
            .get_script_stderr(script_name)
            .expect("failed to get script stderr"),
    };

    let result = TestResult::Verify {
        script_name: script_name.to_string(),
        stream: stream_to_string(stream).into(),
        expected: value_string.to_string(),
        got: got.to_string(),
        success: value_string == got,
    };

    state.add_result(&result);

    Ok(result)
}

fn stream_to_string(stream: &Stream) -> &str {
    match stream {
        Stream::StdOut => "stdout",
        Stream::StdErr => "stderr",
    }
}
