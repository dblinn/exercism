use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Game {
	Loss = 0,
	Draw = 1,
	Win = 2
}

impl Game {
	pub fn parse(result_string: &str) -> Option<Game> {
		match result_string {
			"loss" => Some(Game::Loss),
			"draw" => Some(Game::Draw),
			"win" => Some(Game::Win),
			_ => None
		}
	}

	pub fn opponent_result(&self) -> Game {
		match self {
			&Game::Loss => Game::Win,
			&Game::Draw => Game::Draw,
			&Game::Win => Game::Loss
		}
	}
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

	pub fn add_game(&mut self, game: Game) { self.record.push(game); }

    pub fn games_played(&self) -> usize {
        self.record.len()
    }

    pub fn points(&self) -> usize {
        self.record.iter().cloned().fold(0, |sum, game| sum + game as usize)
    }
}

struct GameOutcome {
	pub away_team: String,
	pub home_team: String,
	away_team_result: Game
}

impl GameOutcome {
	pub fn away_team_result(&self) -> Game { self.away_team_result }
	pub fn home_team_result(&self) -> Game { self.away_team_result.opponent_result() }
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

struct Parser;
impl Parser {
	fn parse(file: File) -> HashMap<String, TeamRecord> {
		let mut records = HashMap::new();
		let reader = BufReader::new(file);
		for score_line in reader.lines() {
			match Self::parse_outcome(score_line) {
				Some(outcome) => {
					records.entry(outcome.away_team)
						.or(TeamRecord::new(outcome.away_team))
						.add_game(outcome.away_team_result);
					records.entry(outcome.home_team)
						.or(TeamRecord::new(outcome.home_team))
						.add_game(outcome.home_team_result);
				},
				_ => {}
			}
		}

		records
	}

	fn parse_outcome(score_line: &str) -> Option<GameOutcome> {
		if (score_line.starts_with('#')) { return None }
		let fields = score_line.split(';');
		if (fields.len() != 3) { return None }
		Game::parse(fields[2])
			.map(|game| GameOutcome { away_team: fields[0], home_team: fields[1], outcome: game })
	}
}

struct Standings {
    records: HashMap<String, TeamRecord>
}

impl Standings {
    pub fn from_file(file: File) -> Standings {
        Standings { records: Parser::parse(file) }
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
