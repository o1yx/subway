use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// Перечисление линий метро
#[derive(Debug)]
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
#[derive(Debug)]
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

    pub fn add_connection(&mut self, to_station: Rc<RefCell<Station>>, travel_time: usize) {
        self.connections.push(Connection::new(Rc::downgrade(&to_station), travel_time));
    }
}

/// Структура связи станций
#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn new_station() {
        let station = Station::new("Subway station 1".to_string(), Lines::Red);
        println!("{:#?}", station);
    }

    #[test]
    fn new_connection() {
        let station_a = Rc::new(RefCell::new(
            Station::new("station_a".to_string(), Lines::Red)
        ));

        let station_b = Rc::new(RefCell::new(
            Station::new("station_b".to_string(), Lines::Red)
        ));

        station_a.borrow_mut().add_connection(station_b.clone(), 7);
        station_b.borrow_mut().add_connection(station_a.clone(), 7);

        println!("{:#?}", station_a);
        println!("{:#?}", station_b);
    }
}
