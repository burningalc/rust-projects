use std::collections::HashMap;

#[derive(Clone, Copy, Eq)]
struct TeamResult {
    points: u32,
    wins: u32,
    draws: u32,
    losses: u32,
}

impl Ord for TeamResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.points.cmp(&self.points)
    }
}

impl PartialOrd for TeamResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TeamResult {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points
    }
}

impl TeamResult {
    fn new() -> TeamResult {
        TeamResult {
            points: 0,
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }
    fn add_record(&mut self, result: MatchResult) {
        match result {
            MatchResult::Win => {
                self.points += 3;
                self.wins += 1;
            }
            MatchResult::Draw => {
                self.points += 1;
                self.draws += 1;
            }
            MatchResult::Loss => {
                self.losses += 1;
            }
        }
    }
}

enum MatchResult {
    Win,
    Draw,
    Loss,
}

pub fn tally(match_results: &str) -> String {
    let results = match_results.split('\n').collect::<Vec<&str>>();
    let mut teams: HashMap<String, TeamResult> = HashMap::new();

    for result in results
        .into_iter()
        .filter(|r| !r.is_empty())
        .collect::<Vec<&str>>()
    {
        let parsed_result = result.split(';').collect::<Vec<&str>>();
        let team_a = parsed_result[0].to_string();
        let team_b = parsed_result[1].to_string();
        match parsed_result[2] {
            "win" => {
                teams
                    .entry(team_a)
                    .or_insert(TeamResult::new())
                    .add_record(MatchResult::Win);
                teams
                    .entry(team_b)
                    .or_insert(TeamResult::new())
                    .add_record(MatchResult::Loss);
            }
            "loss" => {
                teams
                    .entry(team_a)
                    .or_insert(TeamResult::new())
                    .add_record(MatchResult::Loss);
                teams
                    .entry(team_b)
                    .or_insert(TeamResult::new())
                    .add_record(MatchResult::Win);
            }
            "draw" => {
                teams
                    .entry(team_a)
                    .or_insert(TeamResult::new())
                    .add_record(MatchResult::Draw);
                teams
                    .entry(team_b)
                    .or_insert(TeamResult::new())
                    .add_record(MatchResult::Draw);
            }
            _ => {}
        }
    }

    let mut output = String::from("Team                           | MP |  W |  D |  L |  P");
    let mut sorted_teams = teams.into_iter().collect::<Vec<(String, TeamResult)>>();

    // first sort by points, then sort by team name
    sorted_teams.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    for (team_name, team_result) in sorted_teams {
        output.push_str(&format!(
            "\n{:31}| {:2} | {:2} | {:2} | {:2} | {:2}",
            team_name,
            team_result.wins + team_result.draws + team_result.losses,
            team_result.wins,
            team_result.draws,
            team_result.losses,
            team_result.points
        ));
    }

    output
}
