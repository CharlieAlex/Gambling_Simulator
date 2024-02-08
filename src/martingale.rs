use rand::prelude::*;
use pyo3::prelude::*;
// use std::error::Error;
// use csv::Writer;
// use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Outcome {
    Win,
    Lose,
}

impl Outcome {
    fn compute_return(&self, last_stake:f64, odds:f64,) -> f64 {
        match self {
            Outcome::Win => last_stake * odds,
            Outcome::Lose => -last_stake,
        }
    }

    fn compute_decision(&self, last_stake:f64, odds:f64, stake:f64) -> f64 {
        match self {
            Outcome::Win => stake,
            Outcome::Lose => last_stake * odds,
        }
    }
}

struct EndCondition {
    n_simulations: i32,
    max_games: i32,
    init_wealth: f64,
}

struct GameSetting {
    win_rate: f64,
    odds: f64,
    stake: f64,
}

struct GameResult {
    stake_sequence: Vec<f64>,
    return_sequence: Vec<f64>,
    wealth_sequence: Vec<f64>,
}

struct SimulationResult {
    games: Vec<i32>,
    final_wealth: Vec<f64>,
}

#[pyclass]
pub struct OutputResult {
    pub nth: Vec<i32>,
    pub play_times: Vec<i32>,
    pub final_wealth: Vec<i32>,
}

impl OutputResult {
    // fn write_to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
    //     let mut writer = Writer::from_path(file_path)?;
    //     let min_len = self.nth.len().min(self.play_times.len()).min(self.final_wealth.len());

    //     writer.write_record(&["nth", "play_times", "final_wealth"])?;
    //     for i in 0..min_len {
    //         writer.write_record(&[
    //             self.nth[i].to_string(),
    //             self.play_times[i].to_string(),
    //             self.final_wealth[i].to_string(),
    //         ])?;
    //     }

    //     writer.flush()?;
    //     Ok(())
    // }
}

fn play_game(end: &EndCondition, game: &GameSetting, total_result: &mut SimulationResult){
    let mut result = GameResult {
        stake_sequence:  vec![game.stake,],
        return_sequence: vec![0.0,],
        wealth_sequence: vec![end.init_wealth,],
    };

    let mut rng = rand::thread_rng();
    loop {
        // Return in this round
        let random_value: f64 = rng.gen();
        let outcome = match random_value < game.win_rate {
            true => Outcome::Win,
            false => Outcome::Lose,
        };

        // Update return and next stake
        let last_stake = result.stake_sequence.last().unwrap().clone();
        let this_return = outcome.compute_return(last_stake, game.odds);
        let next_stake = outcome.compute_decision(last_stake, game.odds, game.stake);
        let this_wealth = result.wealth_sequence.last().unwrap() + this_return;
        result.return_sequence.push(this_return);
        result.stake_sequence.push(next_stake);
        result.wealth_sequence.push(this_wealth);

        // Check if the game should be terminated
        let n_games = result.return_sequence.len() as i32;
        if (next_stake > this_wealth) | (n_games >= end.max_games){
            total_result.final_wealth.push(this_wealth);
            total_result.games.push(n_games);
            break;
        }
    }
}

pub fn martingale_main() -> OutputResult{
    // parameters
    // let file_path = "/Users/alexlo/Downloads/martingale.csv";
    let end= EndCondition {
        n_simulations: 1_000,
        max_games: 30,
        init_wealth: 10_000.0,
    };
    let game = GameSetting {
        win_rate: 0.5,
        odds: 1.75,
        stake: 100.0,
    };
    let mut total_result = SimulationResult {
        games: vec![],
        final_wealth: vec![],
    };

    // Simulation
    let mut n = 1;
    while n <= end.n_simulations {
        play_game(&end, &game, &mut total_result);
        n += 1;
    }

    // Output
    let final_wealth_i32: Vec<i32> = total_result.final_wealth
        .iter()
        .map(|x| x.round() as i32)
        .collect();
    let df: OutputResult = OutputResult {
        nth: (0..total_result.games.len() as i32).collect(),
        play_times: total_result.games,
        final_wealth: final_wealth_i32,
    };

    // Save result to csv
    // df.write_to_csv(file_path).expect("寫入文件失敗");

    return df
}