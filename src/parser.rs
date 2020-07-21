use super::interpreter::InputCommand;


pub struct Parser;

impl Parser {
    pub fn parse(input: String) -> InputCommand {
        let mut words: Vec<&str> = input.split(" ").collect();
        
        words.reverse();

        let command = words.pop().unwrap();
        match command {
            "to" => {
                let key = words.pop().unwrap();
                return InputCommand::Move(key.to_string());
            },
            "inspect" => return InputCommand::Inpect,
            "quit" => return InputCommand::Quit,
            _ => return InputCommand::Invalid(command.to_string())
        }
    }
}