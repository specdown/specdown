pub use error::Error;
pub use executor::Executor;
pub use run_event::RunEvent;
pub use runnable_action::to_runnable;
pub use state::State;

use crate::types::Action;

mod error;
mod executor;
mod file;
mod run_event;
mod runnable_action;
mod script;
pub mod shell_executor;
mod state;
mod verify;

pub struct Runner<'a> {
    executor: &'a dyn Executor,
    state: &'a mut State,
}

impl<'a> Runner<'a> {
    pub fn create(executor: &'a dyn Executor, state: &'a mut State) -> Self {
        Runner { executor, state }
    }

    pub fn run(&mut self, actions: &[Action]) -> Vec<RunEvent> {
        actions
            .iter()
            .map(|action| self.run_action(action))
            .collect()
    }

    fn run_action(&mut self, action: &Action) -> RunEvent {
        to_runnable(action)
            .run(self.state, &*self.executor)
            .map(|result| {
                self.state.add_result(&result);
                RunEvent::TestCompleted(result)
            })
            .or_else::<Error, _>(|error| Ok(RunEvent::ErrorOccurred(error)))
            .unwrap()
    }
}
