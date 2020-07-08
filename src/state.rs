use super::proc::State;
use std::mem::replace;

// toplevel singleton state manager.
// hold all the state and can only have one instance.
pub struct UnsafeRotonOS {
    state: Option<State<'static>>,
}

impl UnsafeRotonOS {
    pub unsafe fn take_state(&mut self) -> State<'static> {
        let p = replace(&mut self.state, None);
        p.unwrap()
    }
}

pub mod RotonOS {
    // singleton state of the entire os.
    pub static mut ROTON_OS: super::UnsafeRotonOS = super::UnsafeRotonOS {
        state: Some(super::State::new()),
    };
}
