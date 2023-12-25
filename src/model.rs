use std::fmt::{Display, Formatter};

#[allow(dead_code)]
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

#[allow(unused)]
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

#[allow(unused)]
#[derive(Debug, Default, PartialEq, Eq)]
pub enum State {
    #[default]
    Init,
    Dash,
    Edit,
    Quit,
}

#[allow(unused)]
#[derive(Debug)]
pub enum Screen {
    Init,
    Dashboard,
    Edit,
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
#[derive(Debug)]
pub struct Task {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Goal {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct App {
    pub goals: Vec<Goal>,
    pub tasks: Vec<Task>,
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

    pub fn add_task(&mut self, t: Task) {
        self.tasks.push(t)
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

    #[test]
    fn test_add_task() {
        let mut m = App::new();
        assert_eq!(m.tasks.len(), 0);

        let task_name = String::from("a random task");
        let t: Task = Task {
            name: task_name.clone(),
        };
        m.add_task(t);
        assert_eq!(m.tasks.len(), 1);

        assert_eq!(m.tasks[0].name, task_name);
    }
}
