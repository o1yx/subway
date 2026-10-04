use core::fmt;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use serde::{Deserialize};

/// Перечисление линий метро
#[derive(Debug, Deserialize)]
#[repr(u8)]
pub enum Lines {
    Red = 1,
    Blue,
    Green,
    Orange,
    Violet,
    Brown,
}

/// Структура графа метро
/// - `stations` - Вектор ссылок на станции метро, индекс в векторе - id станции
#[derive(Debug)]
struct SubwayGraph {
    stations: Vec<Station>,
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
        let new_station = Station::new(name, line);
        self.stations.push(new_station);
        self.stations.len() - 1
    }

    /// Метод для создания однонаправленной связи между станциями
    fn add_one_way_connection(&mut self, from: usize, to: usize, travel_time: usize) {
        self.stations[from].add_connection(to, travel_time);
    }

    /// Метод для создания двухнаправленной связи между станциями
    pub fn add_two_way_connection(&mut self, station_a: usize, station_b: usize, travel_time: usize) {
        self.add_one_way_connection(station_a, station_b, travel_time);
        self.add_one_way_connection(station_b, station_a, travel_time);
    }
}

impl fmt::Display for SubwayGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.stations.is_empty() {
            return writeln!(f, "Станции отсутствуют");
            
        }

        writeln!(f, "Граф:")?;
        for s in &self.stations {
            writeln!(f, "* {} ({:?})", s.name, s.line)?;
            if s.connections.is_empty() {
                writeln!(f, "- Нет связей")?;
            }

            writeln!(f, "- Связи")?;
            for con in &s.connections {
                writeln!(f, "- - {} ({:?}, {} мин)", self.stations[con.to].name, self.stations[con.to].line, con.travel_time)?;
            }
        }

        Ok(())
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
    fn new(name: String, line: Lines) -> Self {
        Self {
            name,
            line,
            connections: Vec::new(),
        }
    }

    /// Метод для создания односторонней связи станций
    fn add_connection(&mut self, to_station: usize, travel_time: usize) {
        self.connections.push(Connection::new(to_station, travel_time));
    }
}

/// Структура связи станций
/// - `to` - Индекс связанной станции из вектора в SubwayGraph
/// - `travel_time` - Время перемещения до станции
#[derive(Debug)]
struct Connection {
    to: usize,
    travel_time: usize,
}

impl Connection {
    fn new(to_station: usize, travel_time: usize) -> Self {
        Self {
            to: to_station,
            travel_time,
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
        println!("{:?}", station);
    }

    #[test]
    fn new_connection() {
        let mut station_a = Station::new("station_a".to_string(), Lines::Red);

        let mut station_b = Station::new("station_b".to_string(), Lines::Red);

        station_a.add_connection(0, 7);
        station_b.add_connection(1, 7);

        println!("{:?}", station_a);
        println!("{:?}", station_b);
    }

    #[test]
    fn new_subway_graph() {
        let mut graph = SubwayGraph::new();
        let idx_station_a = graph.add_station("Station_A".to_string(), Lines::Red);
        let idx_station_b = graph.add_station("Station_B".to_string(), Lines::Red);
        graph.add_two_way_connection(idx_station_a, idx_station_b, 10);

        println!("{}", graph);
    }
}
