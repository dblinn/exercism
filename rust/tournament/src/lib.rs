use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Write;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Game {
	Loss = 0,
	Draw = 1,
	Win = 2
}

#[derive(Ord, Eq)]
struct TeamRecord {
	pub name: String,
    pub record: Vec<Game>
}

impl TeamRecord {
    pub fn new(name: String) -> TeamRecord {
        TeamRecord { name: name, record: vec![] }
    }

    pub fn games_played(&self) -> usize {
        self.record.len()
    }

    pub fn points(&self) -> usize {
        self.record.iter().cloned().fold(0, |sum, game| sum + game as usize)
    }
}

impl PartialEq for TeamRecord {
    fn eq(&self, other: &TeamRecord) -> bool {
        self.points().eq(&other.points())
    }
}
impl PartialOrd for TeamRecord {
    fn partial_cmp(&self, other: &TeamRecord) -> Option<Ordering> {
        self.points().partial_cmp(&other.points())
    }
}

struct Standings {
    records: HashMap<String, TeamRecord>
}

impl Standings {
    pub fn from_file(file: File) -> Standings {
        Standings { records: HashMap::new() }
    }

    pub fn sorted(&mut self) -> Vec<&TeamRecord> {
		let mut records: Vec<_> = self.records.values().collect();
		records.sort();
		records
    }

    pub fn write_outcome(&self, file: File) {

    }

    pub fn games_played(&self) -> usize {
		// Games are double counted, since two teams play in each game
        let game_count = self.records.iter().fold(0, |total, (_, record)| {
			total + record.games_played()
		});
		game_count / 2
    }
}

pub fn tally(input_path: &Path, output_path: &Path) -> Option<usize> {
    let mut input_file = open_file(input_path);
    let mut output_file = open_file(output_path);
    if input_file.is_none() || output_file.is_none() { return None }

    let mut standings = Standings::from_file(input_file.unwrap());
    standings.write_outcome(output_file.unwrap());

    return Some(standings.games_played());
}

fn open_file(path: &Path) -> Option<File> {
    match File::open(path) {
        Err(e) => { println!("Couldn't open {:?}: {}", path.file_name(), e); None },
        Ok(f) => Some(f)
    }
}
