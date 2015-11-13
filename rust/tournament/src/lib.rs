use std::fs::File;
use std::path::Path;
use std::io::{BufRead, Write, BufReader, BufWriter, Result};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum GameResult {
	Loss = 0,
	Draw = 1,
	Win = 3
}

impl GameResult {
	pub fn parse(result_string: &str) -> Option<GameResult> {
		match result_string {
			"loss" => Some(GameResult::Loss),
			"draw" => Some(GameResult::Draw),
			"win" => Some(GameResult::Win),
			_ => None
		}
	}

	pub fn opponent_result(&self) -> GameResult {
		match self {
			&GameResult::Loss => GameResult::Win,
			&GameResult::Draw => GameResult::Draw,
			&GameResult::Win => GameResult::Loss
		}
	}

	pub fn point_for(&self, point_for_result: GameResult) -> usize {
		if *self == point_for_result { 1 } else { 0 }
	}
}

struct GameSummary {
	pub away_team: String,
	pub home_team: String,
	away_team_result: GameResult
}

impl GameSummary {
	pub fn away_team_result(&self) -> GameResult { self.away_team_result }
	pub fn home_team_result(&self) -> GameResult { self.away_team_result.opponent_result() }
}

#[derive(Eq, Debug)]
struct TeamRecord {
    pub record: Vec<GameResult>
}

impl TeamRecord {
    pub fn new() -> TeamRecord {
        TeamRecord { record: vec![] }
    }

	pub fn add_game(&mut self, game: GameResult) { self.record.push(game); }

    pub fn games_played(&self) -> usize {
        self.record.len()
    }

	fn accumulate_record<F>(&self, game_analyzer: F) -> usize
		where F : Fn(&GameResult) -> usize
	{
		self.record.iter().fold(0, |sum, game| sum + game_analyzer(game))
	}

	pub fn wins(&self) -> usize {
		self.accumulate_record(|game| game.point_for(GameResult::Win))
	}

	pub fn losses(&self) -> usize {
		self.accumulate_record(|game| game.point_for(GameResult::Loss))
	}

	pub fn draws(&self) -> usize {
		self.accumulate_record(|game| game.point_for(GameResult::Draw))
	}

    pub fn points(&self) -> usize {
		self.accumulate_record(|game| *game as usize)
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

struct StandingsParser;
impl StandingsParser {
	fn parse(file: File) -> HashMap<String, TeamRecord> {
		let mut records = HashMap::<String, TeamRecord>::new();
		let reader = BufReader::new(file);

		for score_result in reader.lines() {
			match Self::parse_result(score_result) {
				Some(outcome) => {
					Self::add_outcome(outcome, &mut records)
				},
				_ => {}
			}
		}

		records
	}

	fn add_outcome(outcome: GameSummary, records: &mut HashMap<String, TeamRecord>) {
		let (home_result, away_result) = (
			outcome.home_team_result(),
			outcome.away_team_result()
		);

		records.entry(outcome.home_team).or_insert(TeamRecord::new()).add_game(home_result);
		records.entry(outcome.away_team).or_insert(TeamRecord::new()).add_game(away_result);
	}

	fn parse_result(score_result: Result<String>) -> Option<GameSummary> {
		match score_result {
			Err(_) => None,
			Ok(ref score_line) => Self::parse_game_summary(score_line)
		}
	}

	fn parse_game_summary(score_line: &str) -> Option<GameSummary> {
		if score_line.starts_with('#') { return None }
		let fields = score_line.split(';').collect::<Vec<&str>>();
		if fields.len() != 3 { return None }
		GameResult::parse(fields[2]).map(|game| GameSummary {
			away_team: fields[0].to_string(),
			home_team: fields[1].to_string(),
			away_team_result: game
		})
	}
}

const STANDINGS_HEADER: &'static str =
	"Team                           | MP |  W |  D |  L |  P\n";

struct Standings {
    records: HashMap<String, TeamRecord>
}

impl Standings {
    pub fn from_file(file: File) -> Standings {
        Standings { records: StandingsParser::parse(file) }
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

	pub fn pretty_format(team_name: &str, team_record: &TeamRecord) -> String {
		format!("{:30} | {:2} | {:2} | {:2} | {:2} | {:2}\n",
			team_name,
			team_record.games_played(),
			team_record.wins(),
			team_record.draws(),
			team_record.losses(),
			team_record.points())
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
    let input_file = File::open(input_path)
		.expect("Failed to open input file for reading");
    let output_file = File::create(output_path)
		.expect("Failed to open output file for writing");

    let standings = Standings::from_file(input_file);
	write_standings(&standings, output_file);

    return Some(standings.games_played());
}

fn write_standings(standings: &Standings, file: File) {
	let teams = standings.sorted_teams();
	let mut writer = BufWriter::new(file);
	writer.write(STANDINGS_HEADER.as_bytes()).unwrap();

	for (team_name, team_record) in teams {
		writer.write(Standings::pretty_format(team_name, team_record).as_bytes()).unwrap();
	}
}
