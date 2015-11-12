use std::fmt;

const CORNER: char = '+';
const HORIZONTAL: char = '-';
const VERTICAL: char = '|';

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    pub x: usize,
    pub y: usize
}

struct ContiguousLine {
    pub connector_symbol: char,
    pub points: Vec<Point>
}

impl ContiguousLine {
    pub fn new(connector_symbol: char) -> ContiguousLine {
        ContiguousLine { connector_symbol: connector_symbol, points: vec![] }
    }

    pub fn add(&mut self, symbol: char, x: usize, y: usize) {
        if symbol == CORNER {
            self.points.push(Point { x: x, y: y })
        } else if symbol != self.connector_symbol {
            //println!("Clearing: {} vs {}", symbol, self.connector_symbol);
            self.reset();
        }
    }

    pub fn reset(&mut self) { self.points.clear(); }

    pub fn points(&self) -> &Vec<Point> { &self.points }
}

struct Connector {
    pub coordinates: Point,
    pub right_neighbors: Vec<Point>,
    pub down_neighbors: Vec<Point>
}

impl Connector {
    pub fn new(x: usize, y: usize) -> Connector {
        Connector {
            coordinates: Point { x: x, y: y },
            right_neighbors: Vec::new(),
            down_neighbors: Vec::new()
        }
    }

    pub fn add_neighbors(&mut self,
        horizontal_neighbors: &ContiguousLine,
        vertical_neighbors: &ContiguousLine)
    {
        self.right_neighbors.extend(horizontal_neighbors.points().iter().cloned());
        self.down_neighbors.extend(vertical_neighbors.points().iter().cloned());
    }

    // Returns number of rectangles of which this connector is the upper right corner
    pub fn rectangle_count(&self, connection_topology: &Vec<Vec<Option<Connector>>>) -> usize {
        let mut count = 0;
        println!("{}x{}", self.right_neighbors.len(), self.down_neighbors.len());

        for rn in &self.right_neighbors {
            for dn in &self.down_neighbors {
                let bottom_right = Point { x: rn.x, y: dn.y };
                println!("{:?}", bottom_right);
                count += if let Some(_) = connection_topology[bottom_right.x][bottom_right.y] { 1 } else { 0 }
            }
        }
        count
    }
}

impl fmt::Debug for Connector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.coordinates)
    }
}

struct Topology {
    pub connections: Vec<Vec<Option<Connector>>>
}

impl Topology {
    pub fn new(characters: Vec<Vec<char>>) -> Topology {
        let mut connections: Vec<Vec<Option<Connector>>> = characters.iter().enumerate().map(|(row, line)|
            line.iter().enumerate().map(|(col, c)|
                if *c == '+' { Some(Connector::new(row, col)) } else { None }
            ).collect()
        ).collect();

        Self::read_connections(&mut connections, characters);
        println!("{:?}", connections);
        Topology { connections: connections }
    }

    pub fn rectangle_count(&self) -> usize {
        self.connections.iter().fold(0, |outer_sum, connector_line| {
            connector_line.iter().fold(outer_sum, |inner_sum, connector_option| {
                inner_sum + connector_option.as_ref().map_or(0, |connector| {
                    connector.rectangle_count(&self.connections)
                })
            })
        })
    }

    fn read_connections(connections: &mut Vec<Vec<Option<Connector>>>, characters: Vec<Vec<char>>) {
        let mut horizontal_line = ContiguousLine::new(HORIZONTAL);
        let mut vertical_lines: Vec<ContiguousLine> = characters.iter().map(|_| ContiguousLine::new(VERTICAL)).collect();

        for (x, line) in characters.iter().enumerate().rev() {
            horizontal_line.reset();
            println!("");

            for (y, c) in line.iter().enumerate().rev() {
                for connection in connections[x][y].iter_mut() {
                    connection.add_neighbors(&horizontal_line, &vertical_lines[x]);
                }
                vertical_lines[y].add(*c, x, y);
                println!("{},{}: new size: {}, char was: {}", x, y, vertical_lines[y].points.len(), *c);
                horizontal_line.add(*c, x, y);
                // print!("({},{})", x, y);
            }
        }
        println!("");
    }
}

pub fn count(lines: &[&str]) -> usize {
    let characters: Vec<Vec<char>> = lines.iter()
        .map(|line| line.chars().collect())
        .collect();
    Topology::new(characters).rectangle_count()
}
