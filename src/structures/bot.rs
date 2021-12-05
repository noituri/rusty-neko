use crate::traits::command_trait;
use crate::commands::*;

pub struct Bot {
}

impl Bot {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn commands() -> Vec<Box<dyn command_trait::Command>> {
        vec![
            Box::new(info::ping::PingCommand)
        ]
    }
}

