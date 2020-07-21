use super::{parser, interpreter, room};

use parser::Parser;
use interpreter::Interpreter;
use room::Room;

use std::collections::HashMap;
use console::Term;

pub struct App {
    pub term: Term,
    pub rooms: Box<HashMap<String, Room>>,
    pub room: Option<String>,
    pub quit: bool
}

impl App {
    pub fn new() -> Self {
        App { term: Term::stdout(), room: None, rooms: Box::new(HashMap::new()), quit: false }
    }

    pub fn inspect(&mut self) {
        let current: &str = self.room.as_ref().unwrap();
        let current = self.rooms.get(current).unwrap();

        println!("Current Room: {}", &current.name);
        
        let mut neighbors = String::new();

        for room in current.neighbors.iter() {
            neighbors.push_str(room);
            neighbors.push_str(" ");
        }

        println!("Neighbors: {}", neighbors);
    }

    pub fn move_to(&mut self, room: String) {
        let current: &str = self.room.as_ref().unwrap();
        let current = self.rooms.get(current).unwrap();

        if current.neighbors.contains(&room) {
            let _ = self.term.write_line(format!("Moved to: {}", room).as_ref());
            self.set_room(&room);
        }
        else {
            println!("Cannot move to: {}", room);
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.name.clone(), room);
    }

    pub fn set_room(&mut self, room: &str) {
        self.room = Some(self.rooms.get_mut(room).unwrap().name.clone());
    }

    pub fn process(&mut self, input: String) {
        let command = Parser::parse(input);
        Interpreter::interpret(self, command)
    }

    pub fn create_level(&mut self) {
        let mut lobby = Room::new("Lobby");
        let mut elevator = Room::new("Elevator");
        
        connect_rooms(&mut lobby, &mut elevator);
    
        self.add_room(lobby);
        self.add_room(elevator);

        self.set_room("Lobby");
    }
}

fn connect_rooms(room1: &mut Room, room2: &mut Room) {
    room1.add_neighbor(room2.name.clone());
    room2.add_neighbor(room1.name.clone());
}