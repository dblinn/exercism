use std::fs::File;
use std::path::Path;
use std::io::{BufRead, Write, BufReader, BufWriter, Result};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum Game {
	Loss = 0,
	Draw = 1,
	Win = 3
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

	pub fn point_for(&self, point_for_result: Game) -> usize {
		if *self == point_for_result { 1 } else { 0 }
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

#[derive(Eq, Debug)]
struct TeamRecord {
    pub record: Vec<Game>
}

impl TeamRecord {
    pub fn new() -> TeamRecord {
        TeamRecord { record: vec![] }
    }

	pub fn add_game(&mut self, game: Game) { self.record.push(game); }

    pub fn games_played(&self) -> usize {
        self.record.len()
    }

	pub fn wins(&self) -> usize {
		self.record.iter().fold(0, |sum, game| sum + game.point_for(Game::Win))
	}

	pub fn losses(&self) -> usize {
		self.record.iter().fold(0, |sum, game| sum + game.point_for(Game::Loss))
	}

	pub fn draws(&self) -> usize {
		self.record.iter().fold(0, |sum, game| sum + game.point_for(Game::Draw))
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
impl Ord for TeamRecord {
    fn cmp(&self, other: &TeamRecord) -> Ordering {
        self.points().cmp(&other.points())
    }
}

struct Parser;
impl Parser {
	fn parse(file: File) -> HashMap<String, TeamRecord> {
		let mut records = HashMap::<String, TeamRecord>::new();
		let reader = BufReader::new(file);
		for score_result in reader.lines() {
			match Self::parse_result(score_result) {
				Some(outcome) => {
					let (home_result, away_result, home_team, away_team) = (
						outcome.home_team_result(),
						outcome.away_team_result(),
						outcome.home_team,
						outcome.away_team
					);

					records.entry(home_team)
						.or_insert(TeamRecord::new())
						.add_game(home_result);
					records.entry(away_team)
						.or_insert(TeamRecord::new())
						.add_game(away_result);
				},
				_ => {}
			}
		}

		records
	}

	fn parse_result(score_result: Result<String>) -> Option<GameOutcome> {
		match score_result {
			Err(_) => None,
			Ok(ref score_line) => Self::parse_outcome(score_line)
		}
	}

	fn parse_outcome(score_line: &str) -> Option<GameOutcome> {
		if score_line.starts_with('#') { return None }
		let fields = score_line.split(';').collect::<Vec<&str>>();
		if fields.len() != 3 { return None }
		Game::parse(fields[2]).map(|game| GameOutcome {
			away_team: fields[0].to_string(),
			home_team: fields[1].to_string(),
			away_team_result: game
		})
	}
}

struct Standings {
    records: HashMap<String, TeamRecord>
}

impl Standings {
    pub fn from_file(file: File) -> Standings {
        Standings { records: Parser::parse(file) }
    }

    pub fn sorted_teams(&self) -> Vec<(&String, &TeamRecord)> {
		let mut records: Vec<_> = self.records.iter().collect();
		records.sort_by(|&(name_a, record_a), &(name_b, record_b)|
			match record_b.cmp(record_a) {
				Ordering::Equal => name_a.cmp(name_b),
				unequal @ _ => unequal
			}
		);
		records
    }

	pub fn pretty_print(team_name: &str, team_record: &TeamRecord) -> String {
		format!("{:30} | {:2} | {:2} | {:2} | {:2} | {:2}\n",
			team_name,
			team_record.games_played(),
			team_record.wins(),
			team_record.draws(),
			team_record.losses(),
			team_record.points())
	}

    pub fn write_outcome(&self, file: File) {
		let teams = self.sorted_teams();
		let mut writer = BufWriter::new(file);
		writer.write("Team                           | MP |  W |  D |  L |  P\n".as_bytes()).unwrap();

		for (team_name, team_record) in teams {
			// let line = format!("{:?} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n",
			// writer.write()
			writer.write(Self::pretty_print(team_name, team_record).as_bytes()).unwrap();
		}
		writer.flush().unwrap();
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
    let input_file = open_file(File::open(input_path), input_path);
    let output_file = open_file(File::create(output_path), output_path);
    if input_file.is_none() || output_file.is_none() { return None }

    let standings = Standings::from_file(input_file.unwrap());
    standings.write_outcome(output_file.unwrap());

    return Some(standings.games_played());
}

fn open_file(file_result: Result<File>, path: &Path) -> Option<File> {
    match file_result {
        Err(e) => { println!("Couldn't open {:?}: {}", path.file_name(), e); None },
        Ok(f) => Some(f)
    }
}
