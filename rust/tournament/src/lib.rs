use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy)]
enum Result {
    Win,
    Draw,
    Loss,
}

impl Result {
    fn opposite(&self) -> Self {
        match self {
            Result::Win => Result::Loss,
            Result::Loss => Result::Win,
            Result::Draw => Result::Draw,
        }
    }
}

#[derive(Default, Clone)]
struct Record<'a> {
    name: &'a str,
    wins: usize,
    draws: usize,
    losses: usize,
}

impl<'a> Record<'a> {
    fn new(name: &'a str) -> Self {
        Record {
            name,
            ..Default::default()
        }
    }
    fn points(&self) -> usize {
        3 * self.wins + self.draws
    }
    fn add_result(&mut self, r: Result) {
        match r {
            Result::Win => self.wins += 1,
            Result::Draw => self.draws += 1,
            Result::Loss => self.losses += 1,
        }
    }
}

fn order(x: &&Record, y: &&Record) -> Ordering {
    match (y.points().cmp(&x.points()), y.wins.cmp(&x.wins)) {
        (Ordering::Equal, Ordering::Equal) => x.name.cmp(&y.name),
        (Ordering::Equal, other) => other,
        (other, _) => other,
    }
}

impl<'a> fmt::Display for Record<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.name,
            self.wins + self.draws + self.losses,
            self.wins,
            self.draws,
            self.losses,
            self.points()
        )
    }
}

fn parse_result(s: &str) -> Option<Result> {
    match s {
        "win" => Some(Result::Win),
        "loss" => Some(Result::Loss),
        "draw" => Some(Result::Draw),
        _ => None,
    }
}

pub fn tally<'a>(input: &'a str) -> String {
    let mut records: HashMap<&'a str, Record> = HashMap::new();

    for line in input.lines() {
        let tokens = line.split(';').collect::<Vec<_>>();
        if let [team1, team2, result_str] = &tokens[..] {
            if let Some(result) = parse_result(result_str) {
                records
                    .entry(team1)
                    .or_insert(Record::new(team1))
                    .add_result(result);
                records
                    .entry(team2)
                    .or_insert(Record::new(team2))
                    .add_result(result.opposite());
            }
        }
    }

    let mut sorted: Vec<&Record> = records.values().collect();
    sorted.sort_by(&order);

    let mut result = "Team                           | MP |  W |  D |  L |  P".to_string();
    for record in sorted {
        result.push_str(&format!("\n{}", record));
    }
    result
}
