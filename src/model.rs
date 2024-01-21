#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::{Display, Formatter};

pub enum Message {
    Quit,
    Edit,
    Add,
    Up,
    Down,
    Init,
    Done,
    Cancel,
}

#[derive(Debug)]
pub enum Mode {
    Normal,
    Insert,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum State {
    #[default]
    Init,
    Dash,
    Edit,
    Quit,
}

#[derive(Debug)]
pub enum Screen {
    /// Main screen with list of goals
    Dashboard,
    /// Adding or modifying goals
    Edit,
    /// Prompt to quit application
    Quit,
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Dashboard
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Goal {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct App {
    pub goals: Vec<Goal>,
    pub mode: Mode,
    pub screen: Screen,
    pub state: State,
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn add_goal(&mut self, g: Goal) {
        self.goals.push(g)
    }

    pub fn change_state(&mut self, state: State) -> Option<State> {
        match self.state {
            State::Init if state == State::Dash => Some(State::Dash),
            State::Dash => match state {
                State::Edit if state == State::Edit => Some(State::Edit),
                State::Quit if state == State::Quit => Some(State::Quit),
                _ => Some(self.state),
            },
            State::Edit if state == State::Dash => Some(State::Dash),
            _ => Some(self.state),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_goal() {
        let mut m = App::new();
        assert_eq!(m.goals.len(), 0);

        let goal_name = String::from("a random goal");
        let g: Goal = Goal {
            name: goal_name.clone(),
        };
        m.add_goal(g);
        assert_eq!(m.goals.len(), 1);
        assert_eq!(m.goals[0].name, goal_name);
    }
}
