use std::fmt;
use colored::Colorize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TodoItem {
    pub name: String,
    pub completed: bool,
    pub important: bool,
    pub id: String,
}


impl TodoItem {
    pub fn new(name: String) -> TodoItem {
        TodoItem {
            name: name,
            completed: false,
            important: false,
            id: Uuid::new_v4().to_string()
        }
    }

    pub fn mark_done(&mut self) {
        self.completed = true;
    }

    pub fn mark_not_done(&mut self) {
        self.completed = false;
    }

    pub fn mark_important (&mut self) {
        self.important = true;
    }

    pub fn mark_unimportant (&mut self) {
        self.important = false;
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.important {
            write!(f, "[{}] {}", if self.completed {'\u{2713}'} else {' '}, self.name.red())
        } else {
            write!(f, "[{}] {}", if self.completed {'\u{2713}'} else {' '}, self.name)
        }
    }
}