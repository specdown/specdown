use std::collections::HashMap;

pub struct State {
    number_of_scripts: u32,
    number_of_verifies: u32,
    script_results: HashMap<String, String>,
}

impl State {
    pub fn new() -> State {
        State {
            number_of_scripts: 0,
            number_of_verifies: 0,
            script_results: HashMap::new(),
        }
    }

    pub fn number_of_scripts(&self) -> u32 {
        self.number_of_scripts
    }

    pub fn number_of_verifies(&self) -> u32 {
        self.number_of_verifies
    }

    pub fn add_script_result(&mut self, name: &str, output: &str) {
        self.number_of_scripts += 1;
        self.script_results
            .insert(name.to_string(), output.to_string());
    }

    pub fn get_script_output(&self, name: &str) -> Option<&str> {
        self.script_results.get(name).map(|s| &s[..])
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn sets_counts_to_zero_when_new_state() {
        let state = State::new();
        assert_eq!(state.number_of_scripts(), 0);
        assert_eq!(state.number_of_verifies(), 0);
    }

    #[test]
    fn increments_number_of_scripts_when_script_result_is_added() {
        let mut state = State::new();
        state.add_script_result("script1", "output1");
        assert_eq!(state.number_of_scripts(), 1);
        state.add_script_result("script1", "output2");
        assert_eq!(state.number_of_scripts(), 2);
    }

    #[test]
    fn returns_the_output_when_script_output_exists() {
        let mut state = State::new();
        state.add_script_result("script1", "output1");
        state.add_script_result("script2", "output2");
        assert_eq!(state.get_script_output("script1"), Some("output1"));
        assert_eq!(state.get_script_output("script2"), Some("output2"));
    }

    #[test]
    fn returns_none_when_script_output_does_not_exists() {
        let state = State::new();
        assert_eq!(state.get_script_output("does-not-exist"), None);
    }
}
