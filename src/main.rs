mod app;
mod parser;
mod interpreter;
mod room;

use app::App;

use std::io;

fn main() {
    let mut app = App::new();

    app.create_level();
    
    loop {
        let _ = app.term.write_str(">");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("could not read line");
        
        let input = input.trim().to_string();

        app.process(input);

        if app.quit {
            break;
        }
    }
}

