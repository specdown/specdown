use crate::results::test_result::TestResult;
use crate::runner::state::State;
use crate::types::{ScriptName, Source, VerifyValue};

use super::error::Error;

pub fn run(source: &Source, value: &VerifyValue, state: &mut State) -> Result<TestResult, Error> {
    let Source {
        name: ScriptName(script_name),
        stream: _stream,
    } = source;
    let VerifyValue(value_string) = value;

    let got = state.get_script_output(script_name).expect("failed");

    let result = TestResult::VerifyResult {
        script_name: script_name.to_string(),
        stream: "FIXME output".to_string(),
        expected: value_string.to_string(),
        got: got.to_string(),
        success: value_string == got,
    };

    state.add_result(&result);

    Ok(result)
}
