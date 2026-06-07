use serde::{Serialize, Deserialize};

use core_data_structures::Lines;

#[derive(Debug, Deserialize)]
struct StationData {
    name: String,
    line: Lines,
}

#[derive(Debug, Deserialize)]
struct ConnectionData {
    station_a: usize,
    station_b: usize,
    travel_time: usize,
}

#[derive(Debug, Deserialize)]
struct GraphData {
    stations: Vec<StationData>,
    connections: Vec<ConnectionData>,
}

struct SubwayMapParser {
    graph_data: GraphData
}

impl SubwayMapParser {
    fn new() -> Self {
        SubwayMapParser {
            graph_data: GraphData {
                stations: Vec::new(),
                connections: Vec::new(),
            }
        }
    }

    pub fn load_from_file(&mut self, file_path: String) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::open(file_path)?;
        self.graph_data = serde_json::from_reader(file)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_from_file_work() {
        let mut parser = SubwayMapParser::new();

        parser.load_from_file("../subway_maps/subway_test.json".to_string()).unwrap();

        println!("{:#?}", parser.graph_data);
    }
}

