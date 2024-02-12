mod martingale;
pub use martingale::{EndCondition, GameSetting, SimulationResult, martingale_main};
use std::{collections::HashMap, str::FromStr};
use pyo3::prelude::*;

#[pyfunction]
fn play_martingale(
    n_simulations: i32,
    max_games: i32,
    init_wealth: f64,
    win_rate: f64,
    odds: f64,
    stake: f64,
) -> PyResult<(HashMap<String, Vec<i32>>, HashMap<String, Vec<f64>>)> {
    let end= EndCondition {
        n_simulations,
        max_games,
        init_wealth,
    };
    let game = GameSetting {
        win_rate,
        odds,
        stake,
    };
    let (output_df, lastgame_df) = martingale_main(&end, &game);

    let mut output_map: HashMap<String, Vec<i32>> = HashMap::new();
    output_map.insert(String::from_str("nth").unwrap(), output_df.nth);
    output_map.insert(String::from_str("play_times").unwrap(), output_df.play_times);
    output_map.insert(String::from_str("final_wealth").unwrap(), output_df.final_wealth);

    let mut lastgame_map: HashMap<String, Vec<f64>> = HashMap::new();
    lastgame_map.insert(String::from_str("return").unwrap(), lastgame_df.return_sequence);
    lastgame_map.insert(String::from_str("wealth").unwrap(), lastgame_df.wealth_sequence);
    lastgame_map.insert(String::from_str("stake").unwrap(), lastgame_df.stake_sequence);

    Ok((output_map, lastgame_map))
}

#[pymodule]
fn gambling_simulator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(play_martingale, m)?)?;
    Ok(())
}
