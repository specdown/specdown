use crate::runner::RunEvent;

pub trait Printer: Send {
    fn print(&mut self, event: &RunEvent);
}
