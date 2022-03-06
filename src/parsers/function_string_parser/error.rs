#[derive(Debug, PartialEq)]
pub enum Error {
    MissingArgument {
        function: String,
        argument: String,
    },
    IncorrectArgumentType {
        function: String,
        argument: String,
        expected: String,
        got: String,
    },
}
