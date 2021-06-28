use crate::runner::RunEvent;

pub trait Printer {
    fn print(&mut self, event: &RunEvent);
}
