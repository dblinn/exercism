use std::fmt;

const CORNER: char = '+';
const HORIZONTAL: char = '-';
const VERTICAL: char = '|';

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    pub row: usize,
    pub col: usize
}

struct ContiguousLine {
    pub connector_symbol: char,
    pub points: Vec<Point>
}

impl ContiguousLine {
    pub fn new(connector_symbol: char) -> ContiguousLine {
        ContiguousLine { connector_symbol: connector_symbol, points: vec![] }
    }

    pub fn add(&mut self, symbol: char, row: usize, col: usize) {
        if symbol == CORNER {
            self.points.push(Point { row: row, col: col })
        } else if symbol != self.connector_symbol {
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
    pub fn new(row: usize, col: usize) -> Connector {
        Connector {
            coordinates: Point { row: row, col: col },
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

    pub fn has_below(&self, p: &Point) -> bool {
        self.down_neighbors.contains(p)
    }

    pub fn has_right(&self, p: &Point) -> bool {
        self.right_neighbors.contains(p)
    }

    // Returns number of rectangles of which this connector is the upper right corner
    pub fn rectangle_count(&self, topology: &Topology) -> usize {
        let mut count = 0;
        for rn in &self.right_neighbors {
            for dn in &self.down_neighbors {
                let bottom_right = Point { row: dn.row, col: rn.col };
                count += match topology.connector(&bottom_right) {
                    &Some(_) => {
                        if topology.connector(&rn).as_ref().map_or(false, |c| c.has_below(&bottom_right)) &&
                            topology.connector(&dn).as_ref().map_or(false, |c| c.has_right(&bottom_right)) { 1 }
                        else { 0 }
                    }
                    &None => 0
                }
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
        Topology { connections: connections }
    }

    pub fn connector<'a>(&'a self, p: &Point) -> &'a Option<Connector> {
        &self.connections[p.row][p.col]
    }

    pub fn rectangle_count(&self) -> usize {
        self.connections.iter().fold(0, |outer_sum, connector_line| {
            connector_line.iter().fold(outer_sum, |inner_sum, connector_option| {
                inner_sum + connector_option.as_ref().map_or(0, |connector| {
                    connector.rectangle_count(&self)
                })
            })
        })
    }

    fn read_connections(connections: &mut Vec<Vec<Option<Connector>>>, characters: Vec<Vec<char>>) {
        if connections.len() == 0 { return; }
        let mut horizontal_line = ContiguousLine::new(HORIZONTAL);
        let mut vertical_lines: Vec<ContiguousLine> = characters[0].iter().map(|_| ContiguousLine::new(VERTICAL)).collect();

        for (row, line) in characters.iter().enumerate().rev() {
            horizontal_line.reset();

            for (col, c) in line.iter().enumerate().rev() {
                for connection in connections[row][col].iter_mut() {
                    connection.add_neighbors(&horizontal_line, &vertical_lines[col]);
                }
                vertical_lines[col].add(*c, row, col);
                horizontal_line.add(*c, row, col);
            }
        }
    }
}

pub fn count(lines: &[&str]) -> usize {
    let characters: Vec<Vec<char>> = lines.iter()
        .map(|line| line.chars().collect())
        .collect();
    Topology::new(characters).rectangle_count()
}
