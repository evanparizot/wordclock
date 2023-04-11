pub enum State {
    Time
}

pub struct StateMachine {
    state: State
}

impl StateMachine {
    pub fn new(state: State) -> Self {
        Self { state }
    }

    pub fn next_state(&mut self) {


    }
}