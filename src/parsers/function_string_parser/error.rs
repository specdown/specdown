#[derive(Debug, Eq, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("Function {function} requires argument {argument}")]
    MissingArgument { function: String, argument: String },
    #[error("Function {function} requires argument {argument} to be a {expected}, got {got}")]
    IncorrectArgumentType {
        function: String,
        argument: String,
        expected: String,
        got: String,
    },
}
