use crate::runner::{Error, RunEvent};

pub enum PrintItem {
    RunError(Error),
    RunEvent(RunEvent),
}

pub trait Printer {
    fn print(&mut self, item: &PrintItem);
}
