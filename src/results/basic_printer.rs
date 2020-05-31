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
            TestResult::Script { name, success, .. } => display(&format!(
                "Script {} {}",
                name,
                if *success { "succeeded" } else { "failed" }
            )),
            TestResult::Verify {
                script_name,
                success,
                ..
            } => display(&format!(
                "Verify output from {} {}",
                script_name,
                if *success { "succeeded" } else { "failed" }
            )),
            TestResult::File { path } => display(&format!("File {} created", path)),
        }
    }
}

#[cfg(test)]
mod tests {}
