use crate::traits::command_trait::Command;
use crate::structures::bot::Bot;

#[allow(box_pointers)]
pub fn find_command(cmd: &str) -> Result<Box<dyn Command>, ()> {
    for c in Bot::commands() {
        if c.name().eq(cmd) {
            return Ok(c);
        }
    }

    Err(())
}