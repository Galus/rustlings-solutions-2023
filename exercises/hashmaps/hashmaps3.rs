// hashmaps3.rs

// A list of scores (one per line) of a soccer match is given. Each line
// is of the form :
// <team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>
// Example: England,France,4,2 (England scored 4 goals, France 2).

// You have to build a scores table containing the name of the team, goals
// the team scored, and goals the team conceded. One approach to build
// the scores table is to use a Hashmap. The solution is partially
// written to use a Hashmap, complete it to pass the test.

// Make me pass the tests!

// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a hint.

use std::collections::HashMap;

// A structure to store team name and its goal details.
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

fn debug_scores(scores: &HashMap<String, Team>) {
    for s in scores {
        println!("score: team {}, wins {}, losses {}",
            s.1.name,
            s.1.goals_scored,
            s.1.goals_conceded);
    }
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        // split the thing into lines, this now has all game-matches split into single matches
        let v: Vec<&str> = r.split(',').collect();
        // extract info from each field 
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.
        
        // // add the fields to the scores hashmap
        // let team1 = Team {
        //     name: String::from(&team_1_name),
        //     goals_scored: 0,
        //     goals_conceded: 0,
        // };
        // // We can omit this redundancy of creating two Team's with initial values 
        // // of 0's in another 'clever' way by using the .or_insert_with_key() method.
        // let team2 = Team {
        //     name: String::from(&team_2_name),
        //     goals_scored: 0,
        //     goals_conceded: 0,
        // };
        // // This is special because rather than inserting/overwriting team_1's scores entry,
        // // this will initialize the map with 0's and then for each match adjust the 
        // // scores/conceded values.
        // let team1scores = scores.entry(team_1_name).or_insert(team1);
        // team1scores.goals_scored += team_1_score;
        // team1scores.goals_conceded += team_2_score;

        // let team2scores = scores.entry(team_2_name).or_insert(team2);
        // team2scores.goals_scored += team_2_score;
        // team2scores.goals_conceded += team_1_score;

        // Clever method:
        scores.entry(team_1_name).and_modify(|entry| {
            println!("{} goals_scored adding goals_scored, goals_conceded {}, {} to {}, {}", 
                entry.name, team_1_score, team_2_score, entry.goals_scored, entry.goals_conceded
                );
            entry.goals_scored += team_1_score;
            entry.goals_conceded += team_2_score;
        }).or_insert_with_key( |team| {
            println!("team {} doesnt exist, initializing with 0's", team);
            Team {
                name: String::from(team),
                goals_scored: team_1_score,
                goals_conceded: team_2_score,
            }
        });
        scores.entry(team_2_name).and_modify(|entry| {
            entry.goals_scored += team_2_score;
            entry.goals_conceded += team_1_score;
        }).or_insert_with_key( |team| {
            Team {
                name: String::from(team),
                goals_scored: team_2_score,
                goals_conceded: team_1_score,
            }
        });
    }
    debug_scores(&scores);
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
