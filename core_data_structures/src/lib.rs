use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// Перечисление линий метро
#[repr(u8)]
enum Lines {
    Red = 1,
    Blue,
    Green,
    Orange,
    Violet,
    Brown
}

/// Структура станции
struct Station {
    name: String,                   // Название станции
    line: Lines,                    // Линия метро
    connections: Vec<Connection>    // Связи с другими станциями
}

impl Station {
    pub fn new(name: String, line: Lines) -> Self {
        Self {
            name,
            line,
            connections: Vec::new()
        }
    }
}

/// Структура связи станций
struct Connection {
    to: Weak<RefCell<Station>>,             // Связанная станция
    travel_time: usize,                     // Время перемещения до станции
}

impl Connection {
    fn new(to_station: Weak<RefCell<Station>>, travel_time: usize) -> Self {
        Self {
            to: to_station,
            travel_time
        }
    }
}