use std::process::Command;

#[derive(Debug)]
pub struct CommandParser {}

impl CommandParser {
    pub fn parse_to_command(&self, input: impl AsRef<str>) -> Option<Command> {
        let line_elements: Vec<&str> = input.as_ref().split(" ").collect();

        match line_elements.len() {
            0 => Option::None,
            _ => {
                let mut command = Command::new(line_elements[0]);
                command.args(&line_elements[1..]);

                Option::Some(command)
            }
        }
    }
}
