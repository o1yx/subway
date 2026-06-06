use std::rc::Rc;
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
    to: Rc<RefCell<Station>>,                /// Связанная станция
    travel_time: usize,             // Время перемещения до станции
}

impl Connection {
    pub fn new(to_station: Rc<RefCell<Station>>, travel_time: usize) -> Self {
        Self {
            to: to_station,
            travel_time
        }
    }
}