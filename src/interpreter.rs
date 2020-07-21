use super::App;

pub enum InputCommand {
    Inpect,
    Move(String),
    Quit,
    Invalid(String),
}

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(app: &mut App, command: InputCommand) {
        match command {
            InputCommand::Inpect => app.inspect(),
            InputCommand::Move(room) => app.move_to(room),
            InputCommand::Quit => app.quit = true,
            InputCommand::Invalid(command) => println!("Invalid Command: {}", command)
        }
    }
}