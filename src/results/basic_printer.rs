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
            TestResult::ScriptResult { name, success, .. } => display(&format!(
                "Script {} {}",
                name,
                if *success { "succeeded" } else { "failed" }
            )),
            TestResult::VerifyResult {
                script_name,
                success,
                ..
            } => display(&format!(
                "Verify output from {} {}",
                script_name,
                if *success { "succeeded" } else { "failed" }
            )),
        }
    }
}

#[cfg(test)]
mod tests {}
