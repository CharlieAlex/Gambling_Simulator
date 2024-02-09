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
            Outcome::Win => last_stake*odds - last_stake,
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

pub struct EndCondition {
    pub n_simulations: i32,
    pub max_games: i32,
    pub init_wealth: f64,
}

pub struct GameSetting {
    pub win_rate: f64,
    pub odds: f64,
    pub stake: f64,
}

struct GameResult {
    return_sequence: Vec<f64>,
    wealth_sequence: Vec<f64>,
    stake_sequence: Vec<f64>,
}

pub struct SimulationResult {
    pub games: Vec<i32>,
    pub final_wealth: Vec<f64>,
}

#[pyclass]
#[derive(Debug)]
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
        return_sequence: vec![0.0,],
        wealth_sequence: vec![end.init_wealth,],
        stake_sequence:  vec![game.stake,],
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
        result.wealth_sequence.push(this_wealth);

        // Check if the game should be terminated
        let n_games = result.return_sequence.len() as i32;
        if (this_wealth-next_stake >= 0.) & (n_games < end.max_games){
            result.stake_sequence.push(next_stake);
        } else {
            result.stake_sequence.push(0.);
            total_result.final_wealth.push(this_wealth);
            total_result.games.push(n_games);

            // Check if answers make sense
            if this_wealth <= 10. {
                println!("財富: {:?}", result.wealth_sequence);
                println!("報酬: {:?}", result.return_sequence);
                println!("下注: {:?}", result.stake_sequence);
            }
            break;
        }
    }

    // Check if answers make sense
    // if result.wealth_sequence.last().unwrap() < &1_000.0 {
    //     println!("財富: {:?}", result.wealth_sequence);
    //     println!("報酬: {:?}", result.return_sequence);
    //     println!("下注: {:?}", result.stake_sequence);
    // }
}

pub fn martingale_main(
        end: &EndCondition,
        game: &GameSetting,
    ) -> OutputResult{

    // Result container
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
        play_times: total_result.games.clone(),
        final_wealth: final_wealth_i32,
    };

    // Save result to csv
    // let file_path = "/Users/alexlo/Downloads/martingale.csv";
    // df.write_to_csv(file_path).expect("寫入文件失敗");

    return df
}