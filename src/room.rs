pub struct Room {
    pub name: String,
    pub neighbors: Box<Vec<String>>
}

impl Room {
    pub fn new(name: &str) -> Self {
        Room { name: name.to_string(), neighbors: Box::new(Vec::new()) }
    }

    pub fn add_neighbor(&mut self, room: String) {
        self.neighbors.push(room);
    }
}