use std::collections::BTreeMap;

enum GameResult {
    Win,
    Loss,
    Draw,
}

impl GameResult {
    fn reverse(&self) -> GameResult {
        match self {
            GameResult::Win => GameResult::Loss,
            GameResult::Draw => GameResult::Draw,
            GameResult::Loss => GameResult::Win,
        }
    }

    fn points(&self) -> usize {
        match self {
            GameResult::Win => 3,
            GameResult::Draw => 1,
            GameResult::Loss => 0,
        }
    }
}

struct Game {
    team_1: String,
    team_2: String,
    result: GameResult,
}

#[derive(Eq, PartialEq)]
struct Team {
    name: String,
    matches_played: usize,
    wins: usize,
    draws: usize,
    losses: usize,
    points: usize,
}

impl Team {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            matches_played: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points: 0,
        }
    }

    fn add_result(&mut self, result: &GameResult) {
        self.matches_played += 1;
        match result {
            GameResult::Win => self.wins += 1,
            GameResult::Draw => self.draws += 1,
            GameResult::Loss => self.losses += 1,
        }
        self.points += result.points();
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Team) -> std::cmp::Ordering {
        self.points
            .cmp(&other.points)
            .reverse()
            .then(self.name.cmp(&other.name))
            .then(self.wins.cmp(&other.wins).reverse())
            .then(self.matches_played.cmp(&other.matches_played).reverse())

        // assuming well-formedness, this is enough to conclude that self == other
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Team) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_game_result(result: &str) -> Option<Game> {
    let data: Vec<&str> = result.split(';').map(|datum| datum.trim()).collect();
    if data.len() != 3 {
        return None;
    }

    let result = match data[2].to_ascii_lowercase().as_ref() {
        "win" => GameResult::Win,
        "draw" => GameResult::Draw,
        "loss" => GameResult::Loss,
        _ => return None,
    };

    Some(Game {
        team_1: data[0].to_owned(),
        team_2: data[1].to_owned(),
        result,
    })
}

fn parse_all_results(match_results: &str) -> Vec<Team> {
    let mut results = BTreeMap::new();
    for result in match_results.split('\n').filter(|line| !line.is_empty()) {
        let Game {
            team_1,
            team_2,
            result,
        } = parse_game_result(result).unwrap_or_else(|| panic!("Bad game result: {}", result));
        let new_team_1 = Team::new(&team_1);
        let new_team_2 = Team::new(&team_2);
        results
            .entry(team_1)
            .or_insert(new_team_1)
            .add_result(&result);
        results
            .entry(team_2)
            .or_insert(new_team_2)
            .add_result(&result.reverse());
    }

    results.into_iter().map(|(_k, v)| v).collect()
}

pub fn tally(match_results: &str) -> String {
    let mut board = vec![
        format!("{:<30}", "Team")
            + &format!(" |{:>3}", "MP")
            + &format!(" |{:>3}", "W")
            + &format!(" |{:>3}", "D")
            + &format!(" |{:>3}", "L")
            + &format!(" |{:>3}", "P"),
    ];

    let mut results = parse_all_results(match_results);
    results.sort_unstable();
    for Team {
        name,
        matches_played,
        wins,
        draws,
        losses,
        points,
    } in results
    {
        board.push(
            format!("{:<30}", name)
                + &format!(" |{:>3}", matches_played)
                + &format!(" |{:>3}", wins)
                + &format!(" |{:>3}", draws)
                + &format!(" |{:>3}", losses)
                + &format!(" |{:>3}", points),
        )
    }

    board.join("\n")
}
