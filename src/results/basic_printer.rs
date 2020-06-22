use super::printer::Printer;
use super::test_result::TestResult;

pub struct BasicPrinter {
    display: Box<dyn Fn(&str)>,
}

impl BasicPrinter {
    pub fn new() -> Self {
        Self {
            display: Box::new(|line| println!("{}", line)),
        }
    }
}

impl Printer for BasicPrinter {
    fn print(&self, result: &TestResult) {
        let display = &self.display;
        match result {
            TestResult::Script {
                name,
                success,
                expected_exit_code,
                exit_code,
                ..
            } => {
                let message = if *success {
                    "succeeded".to_string()
                } else {
                    let expected =
                        expected_exit_code.map_or("None".to_string(), |code| code.to_string());
                    let got = exit_code.map_or("None".to_string(), |code| code.to_string());

                    format!("failed (expected exitcode {}, got {})", expected, got)
                };
                display(&format!("Script {} {}", name, message))
            }
            TestResult::Verify {
                script_name,
                success,
                expected,
                got,
                ..
            } => {
                display(&format!(
                    "Verify output from {} {}",
                    script_name,
                    if *success { "succeeded" } else { "failed" }
                ));

                if !*success {
                    display(&format!(
                        "=== Expected ===\n{}\n--- Got ---\n{}\n",
                        expected, got
                    ));
                }
            }
            TestResult::File { path } => display(&format!("File {} created", path)),
        }
    }
}

#[cfg(test)]
mod tests {}
