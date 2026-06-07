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
    Brown,
}

/// Структура графа метро
/// - `stations` - Вектор сильных ссылок на станции метро
#[derive(Debug)]
struct SubwayGraph {
    stations: Vec<Rc<RefCell<Station>>>,
}

impl SubwayGraph {
    pub fn new() -> Self {
        Self {
            stations: Vec::new(),
        }
    }

    /// Метод для создания новой станции, создает smart pointer с подсчетом ссылок и внутренней изменяемостью
    /// # Arguments
    /// - `name` Название станции
    /// - `line` Линия метро этой станции
    /// # Returns
    /// Индекс добавленной станции в векторе `stations`
    pub fn add_station(&mut self, name: String, line: Lines) -> usize {
        let new_station = Rc::new(RefCell::new(Station::new(name, line)));
        self.stations.push(new_station);
        self.stations.len() - 1
    }

    /// Метод для создания однонаправленной связи между станциями
    fn add_one_way_connection(&self, from: usize, to: usize, travel_time: usize) {
        let to_station = self.stations[to].clone();
        self.stations[from].borrow_mut().add_connection(to_station, travel_time);
    }

    /// Метод для создания двухнаправленной связи между станциями
    pub fn add_two_way_connection(&self, station_a: usize, station_b: usize, travel_time: usize) {
        self.add_one_way_connection(station_a, station_b, travel_time);
        self.add_one_way_connection(station_b, station_a, travel_time);
    }
}

/// Структура станции
/// - `name` - Название станции
/// - `line` - Линия метро
/// - `connections` - Вектор связей с другими станциями
#[derive(Debug)]
struct Station {
    name: String,
    line: Lines,
    connections: Vec<Connection>,
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
/// - `to` - Связанная станция
/// - `travel_time` - Время перемещения до станции
#[derive(Debug)]
struct Connection {
    to: Weak<RefCell<Station>>,
    travel_time: usize,
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
